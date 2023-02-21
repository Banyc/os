use core::arch::{asm, global_asm};

use crate::{print, println};

pub fn setup_supervisor_exception_vector() {
    unsafe {
        asm!("la t0, exception_entry");
        asm!("csrw stvec, t0");
    }
}

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn handle_exception() {
    println!("Exception");
    panic!("Not implemented");
}
