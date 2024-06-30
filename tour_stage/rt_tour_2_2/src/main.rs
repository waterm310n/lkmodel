#![no_std]
#![no_main]
#![feature(asm_const)]

#[macro_use]
extern crate axlog2;
extern crate alloc;

mod userboot;
mod trap;

use core::panic::PanicInfo;
use taskctx::TaskState;
use taskctx::PF_KTHREAD;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_2_2]: ...");

    // Setup simplest trap framework.
    trap::init();

    // Prepare for user app to startup.
    userboot::init(cpu_id, dtb_pa);

    // Startup a kernel thread.
    run_queue::init(cpu_id, dtb_pa);

    let ctx = run_queue::spawn_task_raw(2, PF_KTHREAD, || {
        info!("Wander kernel-thread is running ..");
        let ctx = taskctx::current_ctx();
        ctx.set_state(TaskState::Dead);
        info!("Wander kernel-thread yields itself ..");
        run_queue::yield_now();
    });
    run_queue::activate_task(ctx.clone());
    run_queue::yield_now();

    // Load userland app into pgd.
    userboot::load();
    // Start userland app.
    userboot::start();
    userboot::cleanup();

    unreachable!();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
