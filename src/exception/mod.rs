use core::arch::{asm, global_asm};

use crate::{print, println, Sstatus};

pub fn setup_supervisor_exception_handler() {
    unsafe {
        asm!(
            "la t0, exception_entry",
            "csrw stvec, t0",
            //
        );
    }
}

pub fn enable_supervisor_interrupt(interrupt: Interrupt) {
    let sie: usize;
    unsafe {
        asm!("csrr {}, sie", out(reg) sie);
    }
    let sie = sie | (1 << interrupt.exception_code() as usize);
    unsafe {
        asm!("csrw sie, {}", in(reg) sie);
    }
}

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn handle_exception() {
    println!("Exception");

    let frame = ExceptionFrame::new();
    println!("{:#x?}", frame);

    match frame.scause {
        Exception::Interrupt(interrupt) => handle_interrupt(interrupt),
        Exception::Sync(sync_exception) => handle_sync_exception(sync_exception),
    }

    println!("Exception handled");
}

fn handle_interrupt(interrupt: Interrupt) {
    println!("Interrupt: {:?}", interrupt);
    if let Interrupt::Reserved { exception_code } = interrupt {
        panic!("Reserved exception code: {}", exception_code);
    }

    let sip: usize;
    unsafe {
        asm!("csrr {}, sip", out(reg) sip);
    }
    println!("sip: {:#x}", sip);

    // Disable interrupt.
    let sie: usize;
    unsafe {
        asm!("csrr {}, sie", out(reg) sie);
    }
    let sie = sie & !(1 << interrupt.exception_code() as usize);
    unsafe {
        asm!("csrw sie, {}", in(reg) sie);
    }
}

fn handle_sync_exception(sync_exception: SyncException) {
    println!("Sync exception: {:?}", sync_exception);
    if let SyncException::Reserved { exception_code } = sync_exception {
        panic!("Reserved exception code: {}", exception_code);
    }
    increment_sepc();
}

fn increment_sepc() {
    println!("Incrementing sepc");

    let sepc_before: usize;
    unsafe {
        asm!("csrr {}, sepc", out(reg) sepc_before);
    }

    // Set PC to the next instruction.
    let instruction_size = 4;
    unsafe {
        asm!(
            "csrr t0, sepc",
            "add t0, t0, {}",
            "csrw sepc, t0",
            in(reg) instruction_size,
        );
    }

    let sepc_after: usize;
    unsafe {
        asm!("csrr {}, sepc", out(reg) sepc_after);
    }
    println!("sepc: {:#x} -> {:#x}", sepc_before, sepc_after);
}

#[derive(Debug)]
pub struct Cause(usize);

impl Cause {
    pub fn is_interrupt(&self) -> bool {
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

impl Interrupt {
    pub fn exception_code(&self) -> usize {
        match self {
            Interrupt::SupervisorSoftware => 1,
            Interrupt::SupervisorTimer => 5,
            Interrupt::SupervisorExternal => 9,
            Interrupt::Reserved { exception_code } => *exception_code,
            Interrupt::DesignedForPlatformUse { exception_code } => *exception_code,
        }
    }
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
    Trap(Trap),
    Fault(Fault),
    Reserved { exception_code: usize },
    DesignedForPlatformUse { exception_code: usize },
}

impl From<usize> for SyncException {
    fn from(exception_code: usize) -> Self {
        match exception_code {
            0 => SyncException::Fault(Fault::InstructionAddressMisaligned),
            1 => SyncException::Fault(Fault::InstructionAccessFault),
            2 => SyncException::Fault(Fault::IllegalInstruction),
            3 => SyncException::Trap(Trap::Breakpoint),
            4 => SyncException::Fault(Fault::LoadAddressMisaligned),
            5 => SyncException::Fault(Fault::LoadAccessFault),
            6 => SyncException::Fault(Fault::StoreOrAmoAddressMisaligned),
            7 => SyncException::Fault(Fault::StoreOrAmoAccessFault),
            8 => SyncException::Trap(Trap::EnvironmentCallFromUMode),
            9 => SyncException::Trap(Trap::EnvironmentCallFromSMode),
            12 => SyncException::Fault(Fault::InstructionPageFault),
            13 => SyncException::Fault(Fault::LoadPageFault),
            15 => SyncException::Fault(Fault::StoreOrAmoPageFault),
            24..=31 | 48..=63 => SyncException::DesignedForPlatformUse { exception_code },
            _ => SyncException::Reserved { exception_code },
        }
    }
}

#[derive(Debug)]
pub enum Trap {
    Breakpoint,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
}

#[derive(Debug)]
pub enum Fault {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreOrAmoAddressMisaligned,
    StoreOrAmoAccessFault,
    InstructionPageFault,
    LoadPageFault,
    StoreOrAmoPageFault,
}

impl From<Cause> for Exception {
    fn from(cause: Cause) -> Self {
        if cause.is_interrupt() {
            Exception::Interrupt(Interrupt::from(cause.exception_code()))
        } else {
            Exception::Sync(SyncException::from(cause.exception_code()))
        }
    }
}

#[derive(Debug)]
pub struct ExceptionFrame {
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub stval: usize,
    pub scause: Exception,
}

impl ExceptionFrame {
    pub fn new() -> Self {
        let sstatus: usize;
        unsafe {
            asm!("csrr {}, sstatus", out(reg) sstatus);
        }
        let sstatus = Sstatus(sstatus);

        let sepc: usize;
        unsafe {
            asm!("csrr {}, sepc", out(reg) sepc);
        }

        let stval: usize;
        unsafe {
            asm!("csrr {}, stval", out(reg) stval);
        }

        let scause: usize;
        unsafe {
            asm!("csrr {}, scause", out(reg) scause);
        }
        let scause = Cause(scause);
        let scause = Exception::from(scause);

        ExceptionFrame {
            sstatus,
            sepc,
            stval,
            scause,
        }
    }
}
