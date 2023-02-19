#![allow(unused)]

use core::arch::asm;

#[inline(always)]
pub fn sbi_call(ext: &Extension) -> Result<isize, SbiError> {
    let mut error = 0;
    let mut value = 0;
    let e_id = ext.id();
    let f_id = match ext {
        Extension::Base(f) => f.id(),
        _ => 0,
    };
    let arg0 = ext.arg0();
    let arg1: isize = 0;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a6") f_id,
            in("a7") e_id,
        );
    }

    if error == 0 {
        Ok(value)
    } else {
        Err(SbiError::from(error))
    }
}

#[inline(always)]
pub fn legacy_sbi_call(ext: &LegacyExtension) -> Result<isize, isize> {
    let mut err_val = 0;
    let e_id = ext.id();
    let arg0 = ext.arg0();

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => err_val,
            in("a7") e_id,
        );
    }

    if err_val == 0 {
        Ok(err_val)
    } else {
        Err(err_val)
    }
}

#[no_mangle]
pub fn shutdown() -> ! {
    sbi_call(&Extension::Shutdown);
    panic!("Should have been shutdown")
}

pub enum SbiError {
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
    AlreadyStarted = -7,
    AlreadyStopped = -8,
}

impl From<isize> for SbiError {
    fn from(error: isize) -> Self {
        match error {
            -1 => SbiError::Failed,
            -2 => SbiError::NotSupported,
            -3 => SbiError::InvalidParam,
            -4 => SbiError::Denied,
            -5 => SbiError::InvalidAddress,
            -6 => SbiError::AlreadyAvailable,
            -7 => SbiError::AlreadyStarted,
            -8 => SbiError::AlreadyStopped,
            _ => panic!("Unknown SBI error code: {}", error),
        }
    }
}

pub enum LegacyExtension {
    ConsolePutChar { ch: u8 }, // 0x1
    ConsoleGetChar,            // 0x2
}

impl LegacyExtension {
    fn id(&self) -> i32 {
        match self {
            LegacyExtension::ConsolePutChar { .. } => 0x1,
            LegacyExtension::ConsoleGetChar => 0x2,
        }
    }

    fn arg0(&self) -> isize {
        match self {
            LegacyExtension::ConsolePutChar { ch } => *ch as isize,
            _ => 0,
        }
    }
}

pub enum Extension {
    Base(BaseFunction),            // 0x10
    SetTimer { stime_value: u64 }, // 0x54494D45
    SendIpi { hart_mask: usize },  // 0x735049
    Shutdown,                      // 0x53525354
}

impl Extension {
    fn id(&self) -> i32 {
        match self {
            Extension::Base(_) => 0x10,
            Extension::SetTimer { .. } => 0x54494D45,
            Extension::SendIpi { .. } => 0x735049,
            Extension::Shutdown => 0x53525354,
        }
    }

    fn arg0(&self) -> isize {
        match self {
            Extension::Base(f) => f.arg0(),
            Extension::SetTimer { stime_value } => *stime_value as isize,
            Extension::SendIpi { hart_mask } => *hart_mask as isize,
            Extension::Shutdown => 0,
        }
    }
}

pub enum BaseFunction {
    GetSpecVersion,                         // 0
    GetImplId,                              // 1
    GetImplVersion,                         // 2
    ProbeExtension { extension_id: isize }, // 3
    GetMVendorId,                           // 4
    GetMArchId,                             // 5
    GetMImpId,                              // 6
}

impl BaseFunction {
    fn id(&self) -> i32 {
        match self {
            BaseFunction::GetSpecVersion => 0,
            BaseFunction::GetImplId => 1,
            BaseFunction::GetImplVersion => 2,
            BaseFunction::ProbeExtension { .. } => 3,
            BaseFunction::GetMVendorId => 4,
            BaseFunction::GetMArchId => 5,
            BaseFunction::GetMImpId => 6,
        }
    }

    fn arg0(&self) -> isize {
        match self {
            BaseFunction::ProbeExtension { extension_id } => *extension_id,
            _ => 0,
        }
    }
}

pub fn print(s: &str) {
    for ch in s.bytes() {
        let _ = legacy_sbi_call(&LegacyExtension::ConsolePutChar { ch });
    }
}

pub fn println(s: &str) {
    print(s);
    print("\n");
}
