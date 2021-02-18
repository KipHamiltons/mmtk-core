pub(super) mod gc_work;
pub(super) mod global;
pub(super) mod mutator;

pub use self::global::GenCopy;

pub const FULL_NURSERY_GC: bool = false;
pub const NO_SLOW: bool = false;

pub use self::global::GENCOPY_CONSTRAINTS;

use crate::util::side_metadata::*;

const LOGGING_META: SideMetadataSpec = SideMetadataSpec {
   scope: SideMetadataScope::Global,
   offset: 0,
   log_num_of_bits: 0,
   log_min_obj_size: 3,
};