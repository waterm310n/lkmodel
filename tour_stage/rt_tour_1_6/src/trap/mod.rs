//! Trap component.

use riscv::register::stvec;
use riscv::register::scause::{self, Exception as E, Trap};
use axhal::trap::TRAPFRAME_SIZE;
use axhal::arch::TrapFrame;

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
        Trap::Exception(E::Breakpoint) => handle_breakpoint(&mut tf.sepc),
        Trap::Exception(E::UserEnvCall) => handle_linux_syscall(tf),
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

fn handle_breakpoint(sepc: &mut usize) {
    info!("Exception(Breakpoint) @ {:#x} ", sepc);
    *sepc += 2
}

fn handle_linux_syscall(tf: &mut TrapFrame) {
    // Note: "tf.sepc += 4;" must be put before do_syscall. Or:
    // E.g., when we do clone, child task will call clone again
    // and cause strange behavior.
    tf.sepc += 4;
    tf.regs.a0 = do_syscall(tf.regs.a7);
}

const SYS_READ: usize = 63;
const SYS_WRITE:usize = 64;
const SYS_EXIT: usize = 93;

fn do_syscall(sysno: usize) -> usize {
    match sysno {
        SYS_READ => {
            info!("Syscall(Read):");
            0
        },
        SYS_WRITE => {
            info!("Syscall(Write): Hello!");
            0
        },
        SYS_EXIT => {
            info!("Syscall(Exit): system is exiting ...");
            info!("[rt_tour_1_6]: ok!");
            axhal::misc::terminate();
        },
        _ => {
            panic!("Bad sysno: {}", sysno);
        }
    }
}

pub fn init() {
    unsafe {
        stvec::write(trap_vector_base as usize, stvec::TrapMode::Direct)
    }
}
