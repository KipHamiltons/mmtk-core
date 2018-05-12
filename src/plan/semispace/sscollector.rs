use ::plan::CollectorContext;
use ::plan::ParallelCollector;
use ::plan::ParallelCollectorGroup;
use ::plan::semispace;
use ::plan::{phase, Phase};
use ::plan::TraceLocal;
use ::plan::Allocator as AllocationType;

use ::util::alloc::Allocator;
use ::util::alloc::BumpAllocator;
use ::util::{Address, ObjectReference};
use ::util::forwarding_word::clear_forwarding_bits;

use ::policy::copyspace::CopySpace;

use ::util::heap::{PageResource, MonotonePageResource};

use ::vm::{Scanning, VMScanning};

use ::plan::semispace::PLAN;

use super::sstracelocal::SSTraceLocal;

/// per-collector thread behavior and state for the SS plan
pub struct SSCollector {
    pub id: usize,
    // CopyLocal
    pub ss: BumpAllocator<MonotonePageResource<CopySpace>>,
    trace: SSTraceLocal,

    last_trigger_count: usize,
    worker_ordinal: usize,
    group: Option<&'static ParallelCollectorGroup<SSCollector>>,
}

impl CollectorContext for SSCollector {
    fn new() -> Self {
        SSCollector {
            id: 0,
            ss: BumpAllocator::new(0, None),
            trace: SSTraceLocal::new(PLAN.get_sstrace()),

            last_trigger_count: 0,
            worker_ordinal: 0,
            group: None,
        }
    }

    fn init(&mut self, id: usize) {
        self.id = id;
        self.ss.thread_id = id;
        self.trace.init(id);
    }

    fn alloc_copy(&mut self, original: ObjectReference, bytes: usize, align: usize, offset: isize,
                  allocator: AllocationType) -> Address {
        self.ss.alloc(bytes, align, offset)
    }

    fn run(&mut self, thread_id: usize) {
        self.id = thread_id;
        loop {
            self.park();
            self.collect();
        }
    }

    fn collection_phase(&mut self, thread_id: usize, phase: &Phase, primary: bool) {
        match phase {
            &Phase::Prepare => { self.ss.rebind(Some(semispace::PLAN.tospace())) }
            &Phase::StackRoots => {
                trace!("Computing thread roots");
                VMScanning::compute_thread_roots(&mut self.trace, self.id);
                trace!("Thread roots complete");
            }
            &Phase::Roots => {
                trace!("Computing global roots");
                VMScanning::compute_global_roots(&mut self.trace, self.id);
                trace!("Computing static roots");
                VMScanning::compute_static_roots(&mut self.trace, self.id);
                trace!("Finished static roots");
                if super::ss::SCAN_BOOT_IMAGE {
                    trace!("Scanning boot image");
                    VMScanning::compute_bootimage_roots(&mut self.trace, self.id);
                    trace!("Finished boot image");
                }
            }
            &Phase::SoftRefs => {
                // FIXME
            }
            &Phase::WeakRefs => {
                // FIXME
            }
            &Phase::Finalizable => {
                // FIXME
            }
            &Phase::PhantomRefs => {
                // FIXME
            }
            &Phase::ForwardRefs => {
                // FIXME
            }
            &Phase::ForwardFinalizable => {
                // FIXME
            }
            &Phase::Complete => {
            }
            &Phase::Closure => { self.trace.complete_trace() }
            &Phase::Release => { self.trace.release() }
            _ => { panic!("Per-collector phase not handled") }
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn post_copy(&self, object: ObjectReference, rvm_type: Address, bytes: usize, allocator: ::plan::Allocator) {
        clear_forwarding_bits(object);
        match allocator {
            ::plan::Allocator::Los => {
                let unsync = unsafe { &mut *(super::ss::PLAN.unsync.get()) };
                unsync.versatile_space.initialize_header(object); // FIXME: has anotehr parameter: false
            },
            _ => (),
        }
    }
}

impl ParallelCollector for SSCollector {
    type T = SSTraceLocal;

    fn park(&mut self) {
        self.group.unwrap().park(self);
    }

    fn collect(&self) {
        // FIXME use reference instead of cloning everything
        phase::begin_new_phase_stack(self.id, (phase::Schedule::Complex, ::plan::plan::COLLECTION.clone()))
    }

    fn get_current_trace(&mut self) -> &mut SSTraceLocal {
        &mut self.trace
    }

    fn parallel_worker_count(&self) -> usize {
        self.group.unwrap().active_worker_count()
    }

    fn parallel_worker_ordinal(&self) -> usize {
        self.worker_ordinal
    }

    fn rendezvous(&self) -> usize {
        self.group.unwrap().rendezvous()
    }

    fn get_last_trigger_count(&self) -> usize {
        self.last_trigger_count
    }

    fn set_last_trigger_count(&mut self, val: usize) {
        self.last_trigger_count = val;
    }

    fn increment_last_trigger_count(&mut self) {
        self.last_trigger_count += 1;
    }

    fn set_group(&mut self, group: *const ParallelCollectorGroup<Self>) {
        self.group = Some(unsafe { &*group });
    }

    fn set_worker_ordinal(&mut self, ordinal: usize) {
        self.worker_ordinal = ordinal;
    }
}
