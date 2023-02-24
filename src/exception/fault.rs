use super::{ExceptionMutContext, Fault};

pub fn handle_fault(_mut_context: &mut ExceptionMutContext, stval: usize, fault: &Fault) {
    panic!("Fault: {:?}, stval: {}", fault, stval);
}
