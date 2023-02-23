use core::arch::{asm, global_asm};

mod abi_call;

use crate::{supervisor_print, supervisor_println, Sstatus};

pub fn setup_supervisor_exception_handler() {
    unsafe {
        asm!(
            "la t0, __exception_entry",
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
pub extern "C" fn handle_exception(register_context: &mut RegisterContext) {
    supervisor_println!("Exception");

    let mut mut_context = ExceptionMutContext::new(register_context);
    supervisor_println!("{:#x?}", mut_context);

    let immut_context = ExceptionImmutContext::new();
    supervisor_println!("{:#x?}", immut_context);

    match &immut_context.scause {
        Exception::Interrupt(interrupt) => handle_interrupt(&mut mut_context, interrupt),
        Exception::Sync(sync_exception) => handle_sync_exception(&mut mut_context, sync_exception),
    }

    supervisor_println!("Exception handled");
}

fn handle_interrupt(mut_context: &mut ExceptionMutContext, interrupt: &Interrupt) {
    supervisor_println!("Interrupt: {:?}", interrupt);
    if let Interrupt::Reserved { exception_code } = interrupt {
        panic!("Reserved exception code: {}", exception_code);
    }

    // Disable interrupt.
    let sie = mut_context.sie;
    let sie = sie & !(1 << interrupt.exception_code() as usize);
    mut_context.sie = sie;
}

fn handle_sync_exception(mut_context: &mut ExceptionMutContext, sync_exception: &SyncException) {
    supervisor_println!("Sync exception: {:?}", sync_exception);
    if let SyncException::Reserved { exception_code } = sync_exception {
        panic!("Reserved exception code: {}", exception_code);
    }

    if let SyncException::Trap(Trap::Breakpoint) = sync_exception {
        // `ebreak` is just two-bytes long.
        mut_context.sepc += 2;
        return;
    }

    if let SyncException::Trap(Trap::EnvironmentCallFromUMode) = sync_exception {
        abi_call::abi_call(mut_context);
    }
    mut_context.sepc += 4;
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

#[repr(C)]
#[derive(Debug)]
pub struct RegisterContext {
    pub x: [usize; 32],
}

#[derive(Debug)]
pub struct ExceptionMutContext<'entry> {
    pub register_context: &'entry mut RegisterContext,
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub sip: usize,
    pub sie: usize,
}

impl<'entry> ExceptionMutContext<'entry> {
    pub fn new(register_context: &'entry mut RegisterContext) -> Self {
        // Read the context.
        let sstatus: usize;
        let sepc: usize;
        let sip: usize;
        let sie: usize;
        unsafe {
            asm!("csrr {}, sstatus", out(reg) sstatus);
            asm!("csrr {}, sepc", out(reg) sepc);
            asm!("csrr {}, sip", out(reg) sip);
            asm!("csrr {}, sie", out(reg) sie);
        }
        let sstatus = Sstatus(sstatus);

        Self {
            register_context,
            sstatus,
            sepc,
            sip,
            sie,
        }
    }
}

impl Drop for ExceptionMutContext<'_> {
    fn drop(&mut self) {
        // Write the context.
        unsafe {
            asm!("csrw sstatus, {}", in(reg) self.sstatus.0);
            asm!("csrw sepc, {}", in(reg) self.sepc);
            asm!("csrw sip, {}", in(reg) self.sip);
            asm!("csrw sie, {}", in(reg) self.sie);
        }
    }
}

#[derive(Debug)]
pub struct ExceptionImmutContext {
    pub stval: usize,
    pub scause: Exception,
}

impl ExceptionImmutContext {
    pub fn new() -> Self {
        // Read the context.
        let stval: usize;
        let scause: usize;
        unsafe {
            asm!("csrr {}, stval", out(reg) stval);
            asm!("csrr {}, scause", out(reg) scause);
        }
        let scause = Exception::from(Cause(scause));

        ExceptionImmutContext { stval, scause }
    }
}
