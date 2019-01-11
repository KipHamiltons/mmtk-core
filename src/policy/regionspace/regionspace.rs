use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::atomic::Ordering;

use ::util::heap::PageResource;
use ::util::heap::FreeListPageResource;
use ::util::heap::VMRequest;
use ::util::constants::CARD_META_PAGES_PER_REGION;

use ::policy::space::{Space, CommonSpace};
use ::util::{Address, ObjectReference};
use ::plan::TransitiveClosure;
use ::util::forwarding_word as ForwardingWord;
use ::vm::ObjectModel;
use ::vm::VMObjectModel;
use ::plan::Allocator;
use super::region::*;
use util::alloc::embedded_meta_data;
use std::cell::UnsafeCell;
use libc::{c_void, mprotect, PROT_NONE, PROT_EXEC, PROT_WRITE, PROT_READ};
use std::collections::HashSet;
use util::conversions;
use util::constants;
use vm::{Memory, VMMemory};
use util::heap::layout::Mmapper;
use super::DEBUG;
use plan::selected_plan::PLAN;
use plan::plan::Plan;



type PR = FreeListPageResource<RegionSpace>;

#[derive(Debug)]
pub struct RegionSpace {
    common: UnsafeCell<CommonSpace<PR>>,
    // pub regions: RwLock<HashSet<Region>>
}

impl Space for RegionSpace {
    type PR = PR;

    fn common(&self) -> &CommonSpace<Self::PR> {
        unsafe {&*self.common.get()}
    }
    unsafe fn unsafe_common_mut(&self) -> &mut CommonSpace<Self::PR> {
        &mut *self.common.get()
    }

    fn init(&mut self) {
        // Borrow-checker fighting so that we can have a cyclic reference
        let me = unsafe { &*(self as *const Self) };
        if self.vmrequest.is_discontiguous() {
            self.pr = Some(FreeListPageResource::new_discontiguous(METADATA_PAGES_PER_CHUNK));
        } else {
            self.pr = Some(FreeListPageResource::new_contiguous(me, self.start, self.extent, METADATA_PAGES_PER_CHUNK));
        }
        self.pr.as_mut().unwrap().bind_space(me);
    }

    fn is_live(&self, object: ObjectReference) -> bool {
        if ForwardingWord::is_forwarded_or_being_forwarded(object) {
            return true;
        }
        Region::of(object).mark_table.is_marked(object)
    }

    fn is_movable(&self) -> bool {
        true
    }

    fn grow_space(&self, start: Address, bytes: usize, new_chunk: bool) {
        if new_chunk {
            let chunk = conversions::chunk_align(start + bytes, true);
            ::util::heap::layout::heap_layout::MMAPPER.ensure_mapped(chunk, METADATA_PAGES_PER_CHUNK);
            VMMemory::zero(chunk, METADATA_PAGES_PER_CHUNK << constants::LOG_BYTES_IN_PAGE);
        }
    }

    fn release_multiple_pages(&mut self, _start: Address) {
        panic!("not supported")
    }
}

impl RegionSpace {
    pub fn new(name: &'static str, vmrequest: VMRequest) -> Self {
        RegionSpace {
            common: UnsafeCell::new(CommonSpace::new(name, true, false, true, vmrequest)),
            // regions: RwLock::new(HashSet::with_capacity(997)),
        }
    }

    #[inline]
    pub fn acquire_new_region(&self, tls: *mut c_void) -> Option<Region> {
        // Allocate
        let region = self.acquire(tls, PAGES_IN_REGION);

        if !region.is_zero() {
            debug_assert!(region != embedded_meta_data::get_metadata_base(region));
            if DEBUG {
                println!("Alloc {:?} in chunk {:?}", region, embedded_meta_data::get_metadata_base(region));
            }
            // VMMemory::zero(region, BYTES_IN_REGION);
            let mut region = Region(region);
            region.clear();
            region.committed = true;
            // let mut regions = self.regions.write().unwrap();
            // regions.insert(region);
            Some(region)
        } else {
            None
        }
    }

    pub fn prepare(&mut self) {
        let regions = self.regions();
        for region in regions {
            region.clone().mark_table.clear();
            region.live_size.store(0, Ordering::Relaxed);
        }
    }

    pub fn release(&mut self) {
        // Cleanup regions
        let me = unsafe { &mut *(self as *mut Self) };
        // for region in self.regions() {
        //     if region.relocate {
        //         me.release_region(region);
        //     }
        // }
        let to_be_released = {
            let mut to_be_released = vec![];
            for region in self.regions() {
                if region.relocate {
                    to_be_released.push(region);
                }
            }
            to_be_released
        };
        for region in to_be_released {
            me.release_region(region);
        }
    }

    fn release_region(&mut self, mut region: Region) {
        if DEBUG {
            println!("Release {:?}", region);
        }
        region.clear();
        self.pr.as_mut().unwrap().release_pages(region.0);
    }

    #[inline]
    fn test_and_mark(object: ObjectReference, region: Region) -> bool {
        region.mark_table.test_and_mark(object)
    }

    #[inline]
    pub fn trace_mark_object<T: TransitiveClosure>(&self, trace: &mut T, object: ObjectReference) -> ObjectReference {
        let region = Region::of(object);
        if Self::test_and_mark(object, region) {
            region.live_size.fetch_add(VMObjectModel::get_size_when_copied(object), Ordering::Relaxed);
            trace.process_node(object);
        }
        object
    }

    #[inline]
    pub fn trace_evacuate_object<T: TransitiveClosure>(&self, trace: &mut T, object: ObjectReference, allocator: Allocator, tls: *mut c_void) -> ObjectReference {
        let region = Region::of(object);
        if region.relocate {
            let prior_status_word = ForwardingWord::attempt_to_forward(object);
            if ForwardingWord::state_is_forwarded_or_being_forwarded(prior_status_word) {
                ForwardingWord::spin_and_get_forwarded_object(object, prior_status_word)
            } else {
                let new_object = ForwardingWord::forward_object(object, allocator, tls);
                trace.process_node(new_object);
                new_object
            }
        } else {
            if Self::test_and_mark(object, region) {
                trace.process_node(object);
            }
            object
        }
    }

    pub fn compute_collection_set(&self, available_pages: usize) {
        // FIXME: Bad performance
        const MAX_LIVE_SIZE: usize = (BYTES_IN_REGION as f64 * 0.65) as usize;
        let mut regions: Vec<Region> = self.regions().collect();
        regions.sort_unstable_by_key(|r| r.live_size.load(Ordering::Relaxed));
        let avail_regions = (available_pages >> embedded_meta_data::LOG_PAGES_IN_REGION) * REGIONS_IN_CHUNK;
        let mut available_size = avail_regions << LOG_BYTES_IN_REGION;

        for mut region in regions {
            let meta = region.metadata();
            let live_size = meta.live_size.load(Ordering::Relaxed);
            if live_size <= MAX_LIVE_SIZE && live_size < available_size {
                if DEBUG {
                    println!("Relocate {:?}", region);
                }
                meta.relocate = true;
                available_size -= live_size;
            }
        }
        
        // let mut collection_set = regions.drain_filter(|r| r.live_size.load(Ordering::Relaxed) < max_live_size).collect::<Vec<_>>();
        // for region in &mut collection_set {
        //     if DEBUG {
        //         println!("Relocate {:?}", region);
        //     }
        //     region.relocate = true;
        // }
        // debug_assert!(regions.iter().all(|&r| r.live_size.load(Ordering::Relaxed) >= max_live_size));
        // debug_assert!(collection_set.iter().all(|&r| r.live_size.load(Ordering::Relaxed) < max_live_size));
        // collection_set
    }

    #[inline]
    fn regions(&self) -> RegionIterator {
        debug_assert!(!self.contiguous);
        RegionIterator {
            space: unsafe { ::std::mem::transmute(self) },
            cursor: self.head_discontiguous_region,
        }
    }
}

impl ::std::ops::Deref for RegionSpace {
    type Target = CommonSpace<PR>;
    fn deref(&self) -> &CommonSpace<PR> {
        self.common()
    }
}

impl ::std::ops::DerefMut for RegionSpace {
    fn deref_mut(&mut self) -> &mut CommonSpace<PR> {
        self.common_mut()
    }
}

struct RegionIterator {
    space: &'static RegionSpace,
    cursor: Address,
}

impl RegionIterator {
    fn bump_cursor_to_next_region(&mut self) {
        let mut cursor = self.cursor;
        let old_chunk = embedded_meta_data::get_metadata_base(cursor);
        cursor += BYTES_IN_REGION;
        if embedded_meta_data::get_metadata_base(cursor) != old_chunk {
            cursor = ::util::heap::layout::heap_layout::VM_MAP.get_next_contiguous_region(old_chunk);
        }
        self.cursor = cursor;
    }
}

impl Iterator for RegionIterator {
    type Item = Region;
    
    fn next(&mut self) -> Option<Region> {
        if self.cursor.is_zero() {
            return None;
        }
        // Continue searching if `cursor` points to a metadata region
        if self.cursor == embedded_meta_data::get_metadata_base(self.cursor) {
            debug_assert!(::util::heap::layout::heap_layout::VM_MAP.get_descriptor_for_address(self.cursor) == self.space.descriptor);
            self.bump_cursor_to_next_region();
            return self.next();
        }
        // Continue searching if `cursor` points to a free region
        let region = Region(self.cursor);
        if !region.committed {
            self.bump_cursor_to_next_region();
            return self.next();
        }
        debug_assert!(::util::heap::layout::heap_layout::VM_MAP.get_descriptor_for_address(self.cursor) == self.space.descriptor);
        self.bump_cursor_to_next_region();
        Some(region)
        // 
        // let result = RegionSpace::next_region(self.cursor);
        // if let Some(region) = result {
        //     self.cursor = region.0;
        // }
        // result
    }
}

