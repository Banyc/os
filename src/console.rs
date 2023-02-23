use core::fmt::{self, Write};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::sbi_call;

pub fn sbi_print(s: &str) -> Result<(), isize> {
    for ch in s.bytes() {
        let res = sbi_call::legacy_sbi_call(&sbi_call::LegacyExtension::ConsolePutChar { ch });
        match res {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub struct Writer {}

impl Writer {
    pub fn new() -> Self {
        Writer {}
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match sbi_print(s) {
            Ok(_) => Ok(()),
            Err(_) => Err(core::fmt::Error),
        }
    }
}

// Why two writers: to avoid deadlock
lazy_static! {
    pub static ref USER_WRITER: Mutex<Writer> = Mutex::new(Writer::new());
    pub static ref SUPERVISOR_WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

pub fn user_fmt_print(args: fmt::Arguments) {
    USER_WRITER.lock().write_fmt(args).unwrap();
}

pub fn supervisor_fmt_print(args: fmt::Arguments) {
    SUPERVISOR_WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! user_print {
    ($($arg:tt)*) => ($crate::console::user_fmt_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! user_println {
    () => (user_print!("\n"));
    ($($arg:tt)*) => (user_print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! supervisor_print {
    ($($arg:tt)*) => ($crate::console::supervisor_fmt_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! supervisor_println {
    () => (supervisor_print!("\n"));
    ($($arg:tt)*) => (supervisor_print!("{}\n", format_args!($($arg)*)));
}
