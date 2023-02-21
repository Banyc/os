#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::asm;
use core::arch::global_asm;

use os::exception::setup_supervisor_exception_vector;
use os::print;
use os::println;

static HELLO: &str = "Hello World!";

// Entry point of the kernel.
global_asm!(include_str!("_start.asm"));

/// - `no_mangle` ensures the Rust compiler really outputs a function with the name `_start`.
/// - `extern "C"` ensures the Rust compiler uses the C calling convention for this function.
#[no_mangle]
pub extern "C" fn main() {
    println!("{}", HELLO);

    setup_supervisor_exception_vector();

    // We are at supervisor mode now.
    let sstatus: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) sstatus);
    }
    println!("sstatus: {:#x}", sstatus);

    // Accessing machine mode CSR in supervisor mode will cause an exception.
    unsafe {
        asm!("csrr t0, mstatus");
    }

    panic!("Some panic message");
}
