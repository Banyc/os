use core::arch::asm;

use crate::{exception::Interrupt, sbi_call, supervisor_print, supervisor_println};

use super::ExceptionMutContext;

const DELTA: u64 = 10000000;

pub fn handle_interrupt(
    _mut_context: &mut ExceptionMutContext,
    stval: usize,
    interrupt: &Interrupt,
) {
    match interrupt {
        Interrupt::SupervisorSoftware => supervisor_println!("Supervisor software interrupt"),
        Interrupt::SupervisorTimer => {
            supervisor_print!(".");

            let time: u64;
            unsafe {
                asm!("csrr {}, time", out(reg) time);
            }
            sbi_call::set_timer(time + DELTA).expect("Failed to set timer");
        }
        Interrupt::SupervisorExternal => supervisor_println!("Supervisor external interrupt"),
        _ => panic!("Interrupt: {:?}, stval: {}", interrupt, stval),
    }
}
