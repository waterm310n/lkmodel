use riscv::register::sie;
use axhal::arch::TrapFrame;
use core::sync::atomic::{AtomicUsize, Ordering};

/// `Interrupt` bit in `scause`
const INTC_IRQ_BASE: usize = 1 << (usize::BITS - 1);
/// Supervisor timer interrupt in `scause`
const S_TIMER: usize = INTC_IRQ_BASE + 5;
static TICKS: AtomicUsize = AtomicUsize::new(0);

/// Call the external IRQ handler.
pub fn handle(irq_num: usize, _tf: &mut TrapFrame) {
    // Todo: With NoPreempt
    if irq_num == S_TIMER {
        info!("==> Got irq[S_TIMER]");
        update_timer();
        TICKS.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn get_ticks() -> usize {
    TICKS.load(Ordering::Relaxed)
}

fn update_timer() {
    // Setup timer interrupt handler
    const PERIODIC_INTERVAL_NANOS: u64 =
        axhal::time::NANOS_PER_SEC / axconfig::TICKS_PER_SEC as u64 / 10;

    static mut NEXT_DEADLINE: u64 = 0;

    let now_ns = axhal::time::current_time_nanos();
    // Safety: we have disabled preemption in IRQ handler.
    let mut deadline = unsafe { NEXT_DEADLINE };
    if now_ns >= deadline {
        deadline = now_ns + PERIODIC_INTERVAL_NANOS;
    }
    unsafe { NEXT_DEADLINE = deadline + PERIODIC_INTERVAL_NANOS };
    axhal::time::set_oneshot_timer(deadline);
}

pub fn init() {
    unsafe { sie::set_stimer() };
    sbi_rt::set_timer(0);
}
