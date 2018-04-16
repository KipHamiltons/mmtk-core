use ::util::address::Address;
use ::policy::space::*;
use ::vm::{ActivePlan, VMActivePlan};
use ::util::class::*;

use std::marker::PhantomData;
use std::sync::{Mutex, MutexGuard};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::mem::transmute;
use std::fmt::Debug;

static CUMULATIVE_COMMITTED: AtomicUsize = AtomicUsize::new(0);

pub trait PageResource: Sized + 'static + Debug +
        CompleteClass<CommonPageResource<<Self as PageResource>::Space>> {
    // FIXME: For some reason, Rust will refuse to accept calls to methods on an &Self::Space
    // without the 'CompleteMutableClass<CommonSpace<Self>>', despite
    // the 'CompleteMutableClass<CommonSpace<<Self as AbstractSpace>::PR>>' bound on Space
    type Space: Space<PR = Self, This = Self::Space> + CompleteMutableClass<CommonSpace<Self>>;

    /// Allocate pages from this resource.
    /// Simply bump the cursor, and fail if we hit the sentinel.
    /// Return The start of the first page if successful, zero on failure.
    fn get_new_pages(&self, reserved_pages: usize, required_pages: usize, zeroed: bool, thread_id: usize) -> Address {
        self.alloc_pages(reserved_pages, required_pages, zeroed, thread_id)
    }

    // XXX: In the original code reserve_pages & clear_request explicitly
    //      acquired a lock.
    fn reserve_pages(&self, pages: usize) -> usize {
        let adj_pages = self.adjust_for_metadata(pages);
        self.common().reserved.fetch_add(adj_pages, Ordering::Relaxed);
        adj_pages
    }

    fn clear_request(&self, reserved_pages: usize) {
        self.common().reserved.fetch_sub(reserved_pages, Ordering::Relaxed);
    }

    fn update_zeroing_approach(&self, nontemporal: bool, concurrent: bool) {
        debug_assert!(!concurrent || self.common().contiguous);
        unimplemented!()
    }

    fn skip_concurrent_zeroing(&self) {
        unimplemented!()
    }

    fn trigger_concurrent_zeroing(&self) {
        unimplemented!()
    }

    fn concurrent_zeroing(&self) {
        panic!("This PageResource does not implement concurrent zeroing")
    }

    fn alloc_pages(&self, reserved_pages: usize, required_pages: usize, zeroed: bool, thread_id: usize) -> Address;

    fn adjust_for_metadata(&self, pages: usize) -> usize;

    /**
    * Commit pages to the page budget.  This is called after
    * successfully determining that the request can be satisfied by
    * both the page budget and virtual memory.  This simply accounts
    * for the discrepancy between <code>committed</code> and
    * <code>reserved</code> while the request was pending.
    *
    * This *MUST* be called by each PageResource during the
    * allocPages, and the caller must hold the lock.
    */
    fn commit_pages(&self, reserved_pages: usize, actual_pages: usize, thread_id: usize) {
        let delta = actual_pages - reserved_pages;
        self.common().reserved.store(self.common().reserved.load(Ordering::Relaxed) + delta,
                                     Ordering::Relaxed);
        self.common().committed.store(self.common().committed.load(Ordering::Relaxed) + actual_pages,
                                      Ordering::Relaxed);
        if unsafe{VMActivePlan::is_mutator(thread_id)} {
            Self::add_to_committed(actual_pages);
        }
    }

    fn reserved_pages(&self) -> usize {
        self.common().reserved.load(Ordering::Relaxed)
    }

    fn committed_pages(&self) -> usize {
        self.common().committed.load(Ordering::Relaxed)
    }

    fn add_to_committed(pages: usize) {
        CUMULATIVE_COMMITTED.fetch_add(pages, Ordering::Relaxed);
    }


    fn bind_space(&mut self, space: &'static Self::Space) {
        self.common_mut().space = Some(space);
    }
}

pub fn cumulative_committed_pages() -> usize {
    CUMULATIVE_COMMITTED.load(Ordering::Relaxed)
}

#[derive(Debug)]
pub struct CommonPageResource<S: Space> {
    pub reserved: AtomicUsize,
    pub committed: AtomicUsize,

    pub contiguous: bool,
    pub growable: bool,
    pub space: Option<&'static S>,
}