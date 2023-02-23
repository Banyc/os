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

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

pub fn fmt_print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::fmt_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
