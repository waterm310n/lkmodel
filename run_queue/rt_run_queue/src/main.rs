#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use core::panic::PanicInfo;
use taskctx::TaskState::Dead;

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    assert_eq!(cpu_id, 0);

    axlog2::init("debug");
    info!("[rt_run_queue]: ... cpuid {}", cpu_id);

    axhal::arch_init_early(cpu_id);

    axalloc::init();
    page_table::init();

    run_queue::init(cpu_id, dtb_pa);

    let ctx = run_queue::spawn_task_raw(1, || {
        info!("In new task:");
        let ctx = taskctx::current_ctx();
        ctx.set_state(Dead);
        run_queue::yield_now();
    });
    let rq = run_queue::task_rq(&ctx);
    rq.lock().activate_task(ctx.clone());
    rq.lock().resched(false);

    info!("[rt_run_queue]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}
