//! Trap component.

use riscv::register::stvec;
use riscv::register::scause::{self, Exception as E, Trap};
use axhal::trap::TRAPFRAME_SIZE;
use axhal::arch::TrapFrame;

mod syscall;
mod excp;
mod irq;

#[macro_use]
mod macros;
include_asm_marcos!();

core::arch::global_asm!(
    include_str!("trap.S"),
    trapframe_size = const TRAPFRAME_SIZE,
);
extern "C" {
    fn trap_vector_base();
}

#[no_mangle]
pub fn riscv_trap_handler(tf: &mut TrapFrame, _from_user: bool) {
    let scause = scause::read();
    match scause.cause() {
        Trap::Exception(E::Breakpoint) => excp::handle_breakpoint(&mut tf.sepc),
        Trap::Exception(E::UserEnvCall) => syscall::handle(tf),
        Trap::Interrupt(_) => irq::handle(scause.bits(), tf),
        _ => {
            panic!(
                "Unhandled trap {:?} @ {:#x}:\n{:#x?}",
                scause.cause(),
                tf.sepc,
                tf
            );
        }
    }
}

pub fn init() {
    excp::init();
    syscall::init();
    irq::init();

    unsafe {
        stvec::write(trap_vector_base as usize, stvec::TrapMode::Direct)
    }
}
