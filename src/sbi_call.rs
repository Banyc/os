use core::arch::asm;

#[inline(always)]
pub fn sbi_call(ext: &Extension) -> Result<isize, SbiError> {
    let mut error: isize;
    let mut value: isize;
    let e_id = ext.id();
    let f_id = match ext {
        Extension::Base(f) => f.id(),
        _ => 0,
    };
    let arg0 = ext.arg0();
    let arg1 = ext.arg1();

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
    let mut err_val: isize;
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
    sbi_call(&Extension::Shutdown).expect("Failed to shutdown");
    panic!("Should have been shutdown")
}

pub fn set_timer(stime_value: u64) -> Result<(), ()> {
    let res = sbi_call(&Extension::SetTimer { stime_value });
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

#[derive(Debug)]
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

    fn arg1(&self) -> isize {
        match self {
            Extension::SetTimer { stime_value } => match isize::BITS {
                32 => (*stime_value >> 32) as isize,
                64 => 0,
                _ => panic!("Unsupported architecture"),
            },
            _ => 0,
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

pub enum CompatibleSbi {
    Legacy(LegacyExtension),
    Extension(Extension),
}

pub fn decode_sbi_call(a0: usize, a1: usize, a6: usize, a7: usize) -> CompatibleSbi {
    match a7 {
        0x1 => CompatibleSbi::Legacy(LegacyExtension::ConsolePutChar { ch: a0 as u8 }),
        0x2 => CompatibleSbi::Legacy(LegacyExtension::ConsoleGetChar),
        0x10 => match a6 {
            0 => CompatibleSbi::Extension(Extension::Base(BaseFunction::GetSpecVersion)),
            1 => CompatibleSbi::Extension(Extension::Base(BaseFunction::GetImplId)),
            2 => CompatibleSbi::Extension(Extension::Base(BaseFunction::GetImplVersion)),
            3 => CompatibleSbi::Extension(Extension::Base(BaseFunction::ProbeExtension {
                extension_id: a0 as isize,
            })),
            4 => CompatibleSbi::Extension(Extension::Base(BaseFunction::GetMVendorId)),
            5 => CompatibleSbi::Extension(Extension::Base(BaseFunction::GetMArchId)),
            6 => CompatibleSbi::Extension(Extension::Base(BaseFunction::GetMImpId)),
            _ => panic!("Unknown base function"),
        },
        0x54494D45 => CompatibleSbi::Extension(Extension::SetTimer {
            stime_value: (a0 as u64) | ((a1 as u64) << 32),
        }),
        0x735049 => CompatibleSbi::Extension(Extension::SendIpi {
            hart_mask: a0 as usize,
        }),
        0x53525354 => CompatibleSbi::Extension(Extension::Shutdown),
        _ => panic!("Unknown SBI function"),
    }
}
