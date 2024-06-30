#![no_std]
#![no_main]
#![feature(asm_const)]

#[macro_use]
extern crate axlog2;
extern crate alloc;

mod userboot;
mod trap;

use core::panic::PanicInfo;
use taskctx::{TaskState, PF_KTHREAD};

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_2_3]: ...");

    // Setup simplest trap framework.
    trap::init();

    // Startup a kernel thread.
    run_queue::init(cpu_id, dtb_pa);

    let ctx = run_queue::spawn_task_raw(1, 0, move || {
        // Prepare for user app to startup.
        userboot::init(cpu_id, dtb_pa);
        info!("App kernel-thread is running ..");
        // Load userland app into pgd.
        userboot::load();
        run_queue::yield_now();
        // Start userland app.
        userboot::start();
        userboot::cleanup();
        info!("App kernel-thread yields itself ..");
        run_queue::yield_now();
    });
    run_queue::activate_task(ctx.clone());

    let ctx = run_queue::spawn_task_raw(2, PF_KTHREAD, || {
        info!("Wander kernel-thread is running ..");
        let ctx = taskctx::current_ctx();
        ctx.set_state(TaskState::Dead);
        info!("Wander kernel-thread yields itself ..");
        run_queue::yield_now();
    });
    run_queue::activate_task(ctx.clone());

    run_queue::yield_now();

    unreachable!();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
