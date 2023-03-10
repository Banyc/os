#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::asm;
use core::arch::global_asm;

use os::exception::enable_supervisor_interrupt;
use os::exception::setup_supervisor_exception_handler;
use os::sbi_call;
use os::supervisor_print;
use os::supervisor_println;
use os::user_print;
use os::user_println;
use os::Sstatus;

static HELLO: &str = "Hello World!";

// Entry point of the kernel.
global_asm!(include_str!("_start.asm"));

/// - `no_mangle` ensures the Rust compiler really outputs a function with the name `_start`.
/// - `extern "C"` ensures the Rust compiler uses the C calling convention for this function.
#[no_mangle]
pub extern "C" fn main() {
    setup_supervisor_exception_handler();

    supervisor_println!();
    supervisor_println!("{}", HELLO);

    // We are at supervisor mode now.
    let sstatus: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) sstatus);
    }
    let sstatus = Sstatus(sstatus);
    supervisor_println!("{:#x?}", sstatus);

    // Breakpoint
    unsafe {
        asm!("ebreak");
    }

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
    supervisor_println!("sie: {:#x} -> {:#x}", sie_before, sie_after);

    // Trigger timer interrupt.
    sbi_call::set_timer(0).expect("Failed to set timer");

    // Set sepc to the pit.
    unsafe {
        asm!(
            "csrw sepc, {}",
            in (reg) user_pit,
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
    user_println!();
    user_println!("User mode");

    unsafe {
        asm!("ebreak");
    }

    loop {}
}
