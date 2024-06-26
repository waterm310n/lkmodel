use axhal::arch::TrapFrame;
use crate::trap::irq::get_ticks;

const SYS_READ: usize = 63;
const SYS_WRITE:usize = 64;
const SYS_EXIT: usize = 93;

pub fn handle(tf: &mut TrapFrame) {
    // Note: "tf.sepc += 4;" must be put before do_syscall. Or:
    // E.g., when we do clone, child task will call clone again
    // and cause strange behavior.
    tf.sepc += 4;
    tf.regs.a0 = do_syscall(tf.regs.a7, tf.regs.a0);
}

fn do_syscall(sysno: usize, arg0: usize) -> usize {
    match sysno {
        SYS_READ => {
            let ticks = get_ticks();
            info!("Syscall(Read): ticks [{}]", ticks);
            ticks
        },
        SYS_WRITE => {
            let ticks = get_ticks();
            info!("Syscall(Write): ticks [{}:{}]", arg0, ticks);
            0
        },
        SYS_EXIT => {
            info!("Syscall(Exit): system is exiting ...");
            info!("[rt_tour_2_3]: ok!");
            axhal::misc::terminate();
        },
        _ => {
            panic!("Bad sysno: {}", sysno);
        }
    }
}

pub fn init() {
}
