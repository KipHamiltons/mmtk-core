use super::gc_works::MSProcessEdges;
use crate::{mmtk::MMTK, policy::malloc::{self, ALIGN, is_malloced}, util::{Address, constants, heap::layout::vm_layout_constants::{BYTES_IN_CHUNK, LOG_BYTES_IN_CHUNK}, side_metadata}};
use crate::policy::malloc::HEAP_SIZE;
use crate::policy::malloc::ALLOCATION_METADATA_ID;
use crate::policy::malloc::MARKING_METADATA_ID;
use crate::policy::malloc::malloc_usable_size;
use crate::policy::malloc::free;
use crate::policy::malloc::HEAP_USED;
use crate::policy::malloc::MAPPED_CHUNKS;
use crate::policy::mallocspace::MallocSpace;
use crate::plan::global::NoCopy;
use crate::plan::global::BasePlan;
#[cfg(all(feature = "largeobjectspace", feature = "immortalspace"))]
use crate::plan::global::CommonPlan;
use crate::plan::global::GcStatus;
use crate::plan::mutator_context::Mutator;
use crate::plan::mallocms::mutator::create_ms_mutator;
use crate::plan::mallocms::mutator::ALLOCATOR_MAPPING;
use crate::plan::AllocationSemantics;
use crate::plan::Plan;
use crate::policy::space::Space;
use crate::scheduler::gc_works::*;
use crate::scheduler::*;
use crate::util::alloc::allocators::AllocatorSelector;
use crate::util::heap::layout::heap_layout::Mmapper;
use crate::util::heap::layout::heap_layout::VMMap;
use crate::util::heap::layout::vm_layout_constants::{HEAP_END, HEAP_START};
use crate::util::heap::HeapMeta;
use crate::util::options::UnsafeOptionsWrapper;
use crate::util::OpaquePointer;
use crate::util::side_metadata::SideMetadata;
#[cfg(feature = "sanity")]
use crate::util::sanity::sanity_checker::*;
use crate::vm::VMBinding;
use std::{collections::HashSet, sync::Arc};

use atomic::Ordering;
use enum_map::EnumMap;
use malloc::{MIN_OBJECT_SIZE, unset_alloc_bit, unset_mark_bit};

pub type SelectedPlan<VM> = MallocMS<VM>;

pub struct MallocMS<VM: VMBinding> {
    pub base: BasePlan<VM>,
    pub space: MallocSpace<VM>,
}

unsafe impl<VM: VMBinding> Sync for MallocMS<VM> {}

impl<VM: VMBinding> Plan for MallocMS<VM> {
    type VM = VM;
    type Mutator = Mutator<Self>;
    type CopyContext = NoCopy<VM>;

    fn new(
        vm_map: &'static VMMap,
        mmapper: &'static Mmapper,
        options: Arc<UnsafeOptionsWrapper>,
        _scheduler: &'static MMTkScheduler<Self::VM>,
    ) -> Self {
        let heap = HeapMeta::new(HEAP_START, HEAP_END);
        MallocMS {
            base: BasePlan::new(vm_map, mmapper, options, heap),
            space: MallocSpace::new(),
        }
    }



    fn collection_required(&self, _space_full: bool, _space: &dyn Space<Self::VM>) -> bool
    where
            Self: Sized, {
            unimplemented!();
        // unsafe { HEAP_USED.load(Ordering::SeqCst) >= HEAP_SIZE }
    }

    fn gc_init(
        &mut self,
        heap_size: usize,
        vm_map: &'static VMMap,
        scheduler: &Arc<MMTkScheduler<VM>>,
    ) {
        unsafe {
            let align = constants::LOG_BYTES_IN_WORD as usize;
            HEAP_SIZE = heap_size;
            ALLOCATION_METADATA_ID = SideMetadata::request_meta_bits(1, align);
            MARKING_METADATA_ID = SideMetadata::request_meta_bits(1, align);
        }
        self.base.gc_init(heap_size, vm_map, scheduler);
    }

    fn schedule_collection(&'static self, scheduler: &MMTkScheduler<VM>) {
        self.base().set_collection_kind();
        self.base().set_gc_status(GcStatus::GcPrepare);
        // Stop and scan mutators
        scheduler
            .unconstrained_works
            .add(StopMutators::<MSProcessEdges<VM>>::new());
        // Prepare global/collectors/mutators
        scheduler.prepare_stage.add(Prepare::new(self));
        // Release global/collectors/mutators
        scheduler.release_stage.add(Release::new(self));
        // Resume mutators
        #[cfg(feature = "sanity")]
        scheduler.final_stage.add(ScheduleSanityGC);
        scheduler.set_finalizer(Some(EndOfGC));
    }

    fn bind_mutator(
        &'static self,
        tls: OpaquePointer,
        _mmtk: &'static MMTK<Self::VM>,
    ) -> Box<Mutator<Self>> {
        Box::new(create_ms_mutator(tls, self))
    }
    
    fn get_allocator_mapping(&self) -> &'static EnumMap<AllocationSemantics, AllocatorSelector> {
        &*ALLOCATOR_MAPPING
    }

    fn prepare(&self, _tls: OpaquePointer) {
        // Do nothing  
    }

    fn release(&self, _tls: OpaquePointer) {
        println!("Begin release: HEAP_USED = {}", HEAP_USED.load(Ordering::SeqCst));
        unsafe {
            let chunks = &*MAPPED_CHUNKS.read().unwrap();
            // println!("num chunks mapped = {}", chunks.len());
            for chunk_start in chunks {
                let mut address = *chunk_start;
                let end_of_chunk = chunk_start.add(BYTES_IN_CHUNK);
                while address.as_usize() < end_of_chunk.as_usize() {
                    if SideMetadata::load_atomic(ALLOCATION_METADATA_ID, address) == 1 {
                        if SideMetadata::load_atomic(MARKING_METADATA_ID, address) == 0 {
                            let ptr = address.to_mut_ptr();
                            let freed_memory = malloc_usable_size(ptr);
                            HEAP_USED.fetch_sub(freed_memory, Ordering::SeqCst);
                            free(ptr);
                            unset_alloc_bit(address);
                        } else {
                            unset_mark_bit(address);
                        }
                    }
                    address = address.add(ALIGN);
                }
            }
        }
        println!("Done release: HEAP_USED = {}", HEAP_USED.load(Ordering::SeqCst));
    }

    fn get_collection_reserve(&self) -> usize {
        unimplemented!();
    }

    fn get_pages_used(&self) -> usize {
        self.base.get_pages_used()
    }

    fn base(&self) -> &BasePlan<VM> {
        &self.base
    }

    #[cfg(all(feature = "largeobjectspace", feature = "immortalspace"))]
    fn common(&self) -> &CommonPlan<VM> {
        unreachable!("MallocMS does not have a common plan.");
    }
}