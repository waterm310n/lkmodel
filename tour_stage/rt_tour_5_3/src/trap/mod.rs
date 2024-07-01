//! Trap component.

use riscv::register::stvec;
use riscv::register::scause::{self, Exception as E, Trap};
use axhal::trap::TRAPFRAME_SIZE;
use axhal::arch::TrapFrame;

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
        Trap::Interrupt(_) => irq::handle(scause.bits(), tf),
        _ => {
            info!("No Trap handler! {:?} @ {:#x}", scause.cause(), tf.sepc);
            info!("[rt_tour_5_3]: ok!");
            axhal::misc::terminate();
        }
    }
}

pub fn init() {
    unsafe {
        stvec::write(trap_vector_base as usize, stvec::TrapMode::Direct)
    }
}
