#![no_std] // don't link the Rust standard library

pub mod exception;
pub mod io;
pub mod sbi_call;

use core::panic::PanicInfo;

/// - This function is called on panic.
/// - `!` means this function never returns.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    sbi_call::shutdown();
}
