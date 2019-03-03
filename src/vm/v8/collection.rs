use super::super::Collection;
use ::plan::{MutatorContext, ParallelCollector};

use libc::c_void;

pub struct VMCollection {}

impl Collection for VMCollection {
    fn stop_all_mutators(tls: *mut c_void) {
        unimplemented!();
    }

    fn resume_mutators(tls: *mut c_void) {
        unimplemented!();
    }

    fn block_for_gc(tls: *mut c_void) {
        unimplemented!();
    }

    unsafe fn spawn_worker_thread<T: ParallelCollector>(tls: *mut c_void, ctx: *mut T) {
        unimplemented!();
    }

    fn prepare_mutator<T: MutatorContext>(tls: *mut c_void, m: &T) {
        unimplemented!()
    }
}