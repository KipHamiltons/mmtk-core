use super::gc_work::*;
use super::GenCopy;
use crate::plan::barriers::*;
use crate::plan::mutator_context::Mutator;
use crate::plan::mutator_context::MutatorConfig;
use crate::plan::AllocationSemantics as AllocationType;
use crate::util::alloc::allocators::{AllocatorSelector, Allocators};
use crate::util::alloc::BumpAllocator;
use crate::util::{VMMutatorThread, VMWorkerThread};
use crate::vm::{ObjectModel, VMBinding};
use crate::MMTK;
use enum_map::enum_map;
use enum_map::EnumMap;

pub fn gencopy_mutator_prepare<VM: VMBinding>(_mutator: &mut Mutator<VM>, _tls: VMWorkerThread) {
    // Do nothing
}

pub fn gencopy_mutator_release<VM: VMBinding>(mutator: &mut Mutator<VM>, _tls: VMWorkerThread) {
    // reset nursery allocator
    let bump_allocator = unsafe {
        mutator
            .allocators
            .get_allocator_mut(mutator.config.allocator_mapping[AllocationType::Default])
    }
    .downcast_mut::<BumpAllocator<VM>>()
    .unwrap();
    bump_allocator.reset();
}

#[cfg(not(feature = "force_vm_spaces"))]
lazy_static! {
    pub static ref ALLOCATOR_MAPPING: EnumMap<AllocationType, AllocatorSelector> = enum_map! {
        AllocationType::Default => AllocatorSelector::BumpPointer(0),
        AllocationType::Immortal | AllocationType::Code | AllocationType::LargeCode | AllocationType::ReadOnly => AllocatorSelector::BumpPointer(1),
        AllocationType::Los => AllocatorSelector::LargeObject(0),
    };
}

#[cfg(feature = "force_vm_spaces")]
lazy_static! {
    #[cfg(feature = "force_vm_spaces")]
    pub static ref ALLOCATOR_MAPPING: EnumMap<AllocationType, AllocatorSelector> = enum_map! {
        AllocationType::Default => AllocatorSelector::BumpPointer(0),
        AllocationType::Immortal => AllocatorSelector::BumpPointer(1),
        AllocationType::ReadOnly => AllocatorSelector::BumpPointer(2),
        AllocationType::Code => AllocatorSelector::BumpPointer(3),
        AllocationType::LargeCode => AllocatorSelector::BumpPointer(4),
        AllocationType::Los => AllocatorSelector::LargeObject(0),
    };
}

pub fn create_gencopy_mutator<VM: VMBinding>(
    mutator_tls: VMMutatorThread,
    mmtk: &'static MMTK<VM>,
) -> Mutator<VM> {
    let gencopy = mmtk.plan.downcast_ref::<GenCopy<VM>>().unwrap();
    let config = MutatorConfig {
        allocator_mapping: &*ALLOCATOR_MAPPING,
        space_mapping: box vec![
            (AllocatorSelector::BumpPointer(0), &gencopy.nursery),
            (
                AllocatorSelector::BumpPointer(1),
                gencopy.common.get_immortal(),
            ),
            (AllocatorSelector::LargeObject(0), gencopy.common.get_los()),
            #[cfg(all(feature = "force_vm_spaces", feature = "ro_space"))]
            (
                AllocatorSelector::BumpPointer(2),
                &gencopy.common.base.ro_space,
            ),
            #[cfg(all(feature = "force_vm_spaces", feature = "code_space"))]
            (
                AllocatorSelector::BumpPointer(3),
                &gencopy.common.base.code_space,
            ),
            #[cfg(all(feature = "force_vm_spaces", feature = "code_space"))]
            (
                AllocatorSelector::BumpPointer(4),
                &gencopy.common.base.code_lo_space,
            ),
        ],
        prepare_func: &gencopy_mutator_prepare,
        release_func: &gencopy_mutator_release,
    };

    Mutator {
        allocators: Allocators::<VM>::new(mutator_tls, &*mmtk.plan, &config.space_mapping),
        barrier: box ObjectRememberingBarrier::<GenCopyNurseryProcessEdges<VM>>::new(
            mmtk,
            *VM::VMObjectModel::GLOBAL_LOG_BIT_SPEC,
        ),
        mutator_tls,
        config,
        plan: gencopy,
    }
}
