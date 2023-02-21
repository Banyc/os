use core::arch::{asm, global_asm};

use crate::{print, println};

pub fn setup_supervisor_exception_handler() {
    unsafe {
        asm!("la t0, exception_entry");
        asm!("csrw stvec, t0");
    }
}

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn handle_exception() {
    println!("Exception");

    let scause: usize;
    let sepc: usize;
    unsafe {
        asm!("csrr {}, scause", out(reg) scause);
        asm!("csrr {}, sepc", out(reg) sepc);
    }
    let scause = Cause(scause);

    println!("scause: {:#x}", scause.0);
    println!("sepc: {:#x}", sepc);

    let exception = Exception::from(scause);
    match exception {
        Exception::Interrupt(interrupt) => handle_interrupt(interrupt),
        Exception::Sync(sync_exception) => handle_sync_exception(sync_exception),
    }
}

fn handle_interrupt(interrupt: Interrupt) {
    println!("Interrupt: {:?}", interrupt);
    if let Interrupt::Reserved { exception_code } = interrupt {
        panic!("Reserved exception code: {}", exception_code);
    }
}

fn handle_sync_exception(sync_exception: SyncException) {
    println!("Sync exception: {:?}", sync_exception);
    if let SyncException::Reserved { exception_code } = sync_exception {
        panic!("Reserved exception code: {}", exception_code);
    }
    panic!("Not implemented");
}

pub struct Cause(usize);

impl Cause {
    pub fn interrupt(&self) -> bool {
        match usize::BITS {
            32 => self.0 & 0x8000_0000 != 0,
            64 => self.0 & 0x8000_0000_0000_0000 != 0,
            _ => unreachable!(),
        }
    }

    pub fn exception_code(&self) -> usize {
        match usize::BITS {
            32 => self.0 & 0x7fff_ffff,
            64 => self.0 & 0x7fff_ffff_ffff_ffff,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Exception {
    Interrupt(Interrupt),
    Sync(SyncException),
}

#[derive(Debug)]
pub enum Interrupt {
    SupervisorSoftware,
    SupervisorTimer,
    SupervisorExternal,
    Reserved { exception_code: usize },
    DesignedForPlatformUse { exception_code: usize },
}

impl From<usize> for Interrupt {
    fn from(exception_code: usize) -> Self {
        match exception_code {
            1 => Interrupt::SupervisorSoftware,
            5 => Interrupt::SupervisorTimer,
            9 => Interrupt::SupervisorExternal,
            0 | 2..=4 | 6..=8 | 10..=15 => Interrupt::Reserved { exception_code },
            _ => Interrupt::DesignedForPlatformUse { exception_code },
        }
    }
}

#[derive(Debug)]
pub enum SyncException {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreOrAmoAddressMisaligned,
    StoreOrAmoAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    InstructionPageFault,
    LoadPageFault,
    StoreOrAmoPageFault,
    Reserved { exception_code: usize },
    DesignedForPlatformUse { exception_code: usize },
}

impl From<usize> for SyncException {
    fn from(exception_code: usize) -> Self {
        match exception_code {
            0 => SyncException::InstructionAddressMisaligned,
            1 => SyncException::InstructionAccessFault,
            2 => SyncException::IllegalInstruction,
            3 => SyncException::Breakpoint,
            4 => SyncException::LoadAddressMisaligned,
            5 => SyncException::LoadAccessFault,
            6 => SyncException::StoreOrAmoAddressMisaligned,
            7 => SyncException::StoreOrAmoAccessFault,
            8 => SyncException::EnvironmentCallFromUMode,
            9 => SyncException::EnvironmentCallFromSMode,
            12 => SyncException::InstructionPageFault,
            13 => SyncException::LoadPageFault,
            15 => SyncException::StoreOrAmoPageFault,
            24..=31 | 48..=63 => SyncException::DesignedForPlatformUse { exception_code },
            _ => SyncException::Reserved { exception_code },
        }
    }
}

impl From<Cause> for Exception {
    fn from(cause: Cause) -> Self {
        if cause.interrupt() {
            Exception::Interrupt(Interrupt::from(cause.exception_code()))
        } else {
            Exception::Sync(SyncException::from(cause.exception_code()))
        }
    }
}
