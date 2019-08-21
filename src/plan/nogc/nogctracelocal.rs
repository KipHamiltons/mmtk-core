use ::plan::transitive_closure::TransitiveClosure;
use ::util::address::{Address, ObjectReference};
use ::plan::tracelocal::TraceLocal;

pub struct NoGCTraceLocal {}

impl TransitiveClosure for NoGCTraceLocal {
    fn process_edge(&mut self, _src: ObjectReference, _slot: Address) {
        unimplemented!();
    }

    fn process_node(&mut self, object: ObjectReference) {
        unimplemented!()
    }
}

impl TraceLocal for NoGCTraceLocal {
    fn process_roots(&mut self) {
        unimplemented!();
    }

    fn process_root_edge(&mut self, slot: Address, untraced: bool) {
        unimplemented!();
    }

    fn trace_object(&mut self, object: ObjectReference) -> ObjectReference {
        unimplemented!();
    }

    fn complete_trace(&mut self) {
        unimplemented!();
    }

    fn release(&mut self) {
        unimplemented!();
    }

    fn process_interior_edge(&mut self, target: ObjectReference, slot: Address, root: bool) {
        unimplemented!()
    }
    fn report_delayed_root_edge(&mut self, slot: Address) {
        unimplemented!()
    }

    fn will_not_move_in_current_collection(&self, obj: ObjectReference) -> bool {
        true
    }

    fn is_live(&self, object: ObjectReference) -> bool {
        return true;
    }
}

impl NoGCTraceLocal {
    pub fn new() -> Self {
        Self {}
    }
}