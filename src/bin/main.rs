#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::asm;
use core::arch::global_asm;

use os::exception::enable_supervisor_interrupt;
use os::exception::setup_supervisor_exception_handler;
use os::print;
use os::println;
use os::sbi_call;
use os::Sstatus;

static HELLO: &str = "Hello World!";

// Entry point of the kernel.
global_asm!(include_str!("_start.asm"));

/// - `no_mangle` ensures the Rust compiler really outputs a function with the name `_start`.
/// - `extern "C"` ensures the Rust compiler uses the C calling convention for this function.
#[no_mangle]
pub extern "C" fn main() {
    println!();
    println!("{}", HELLO);

    // We are at supervisor mode now.
    let sstatus: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) sstatus);
    }
    let sstatus = Sstatus(sstatus);
    println!("{:#x?}", sstatus);

    setup_supervisor_exception_handler();

    // Accessing machine mode CSR in supervisor mode will cause an exception.
    unsafe {
        asm!("csrr t0, mstatus");
    }

    // We are still at supervisor mode now.
    let sstatus: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) sstatus);
    }
    println!("sstatus: {:#x}", sstatus);

    // Enable timer interrupt.
    let sie_before: usize;
    unsafe {
        asm!("csrr {}, sie", out(reg) sie_before);
    }
    enable_supervisor_interrupt(os::exception::Interrupt::SupervisorTimer);
    let sie_after: usize;
    unsafe {
        asm!("csrr {}, sie", out(reg) sie_after);
    }
    println!("sie: {:#x} -> {:#x}", sie_before, sie_after);

    // Trigger timer interrupt.
    sbi_call::set_timer(0).expect("Failed to set timer");

    // Set sepc to the pit.
    unsafe {
        asm!(
            "la t0, user_pit",
            "csrw sepc, t0",
            //
        );
    }

    // Go to user mode.
    // Send the PC to the pit.
    unsafe {
        asm!("sret");
    }

    panic!("Should not reach here");
}

#[no_mangle]
pub extern "C" fn user_pit() -> ! {
    loop {
        // Wait for interrupt.
    }
}
