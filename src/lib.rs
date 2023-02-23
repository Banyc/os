#![no_std] // don't link the Rust standard library

pub mod console;
pub mod exception;
pub mod sbi_call;

use core::{fmt, panic::PanicInfo};

/// - This function is called on panic.
/// - `!` means this function never returns.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    supervisor_println!("{}", info);
    sbi_call::shutdown();
}

pub struct Sstatus(pub usize);

impl Sstatus {
    /// SIE
    pub fn is_interrupt_enabled(&self) -> bool {
        self.0 & 1 << 1 != 0
    }

    /// SPIE
    pub fn is_interrupt_enabled_before_exception(&self) -> bool {
        self.0 & 1 << 5 != 0
    }

    /// UBE
    pub fn is_user_big_endian(&self) -> bool {
        self.0 & 1 << 6 != 0
    }

    /// SPP
    pub fn mode_before_exception(&self) -> Spp {
        let spp = (self.0 >> 8) & 1;
        Spp::from(spp)
    }
}

impl fmt::Debug for Sstatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sstatus")
            .field("raw", &self.0)
            .field("is_interrupt_enabled", &self.is_interrupt_enabled())
            .field(
                "is_interrupt_enabled_before_exception",
                &self.is_interrupt_enabled_before_exception(),
            )
            .field("is_user_big_endian", &self.is_user_big_endian())
            .field("mode_before_exception", &self.mode_before_exception())
            .finish()
    }
}

#[derive(Debug)]
pub enum Spp {
    Supervisor,
    User,
}

impl From<usize> for Spp {
    fn from(value: usize) -> Self {
        match value {
            0 => Spp::User,
            1 => Spp::Supervisor,
            _ => unreachable!(),
        }
    }
}
