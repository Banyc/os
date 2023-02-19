#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::global_asm;

use os::print;
use os::println;

static HELLO: &str = "Hello World!";

// Entry point of the kernel.
global_asm!(include_str!("asm/_start.asm"));

/// - `no_mangle` ensures the Rust compiler really outputs a function with the name `_start`.
/// - `extern "C"` ensures the Rust compiler uses the C calling convention for this function.
#[no_mangle]
pub extern "C" fn main() {
    println!("{}", HELLO);
    panic!("Some panic message");
}
