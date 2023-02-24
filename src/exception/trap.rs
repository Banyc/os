use crate::{supervisor_print, supervisor_println};

use super::{abi_call, ExceptionMutContext, Trap};

pub fn handle_trap(mut_context: &mut ExceptionMutContext, stval: usize, trap: &Trap) {
    match trap {
        Trap::Breakpoint => {
            supervisor_println!("Breakpoint");

            // `ebreak` is just two-bytes long.
            mut_context.sepc += 2;
        }
        Trap::EnvironmentCallFromUMode => {
            abi_call::abi_call(mut_context);
            mut_context.sepc += 4;
        }
        _ => panic!("Trap: {:?}, stval: {}", trap, stval),
    }
}
