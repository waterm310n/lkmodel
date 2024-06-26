//! Trap component.

use riscv::register::stvec;

core::arch::global_asm!(
    include_str!("trap.S")
);
extern "C" {
    fn trap_vector_base();
}

#[no_mangle]
pub fn riscv_trap_handler() {
    error!("As expected: NO trap handler!");
    info!("[rt_tour_1_5]: ok!");
    axhal::misc::terminate();
}

pub fn init() {
    unsafe {
        stvec::write(trap_vector_base as usize, stvec::TrapMode::Direct)
    }
}
