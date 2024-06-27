#![no_std]
#![no_main]
#![feature(asm_const)]

#[macro_use]
extern crate axlog2;
extern crate alloc;

mod userboot;
mod trap;

use core::panic::PanicInfo;
use trap::PERIODIC_INTERVAL_NANOS;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("info");
    info!("[rt_tour_2_4]: ...");

    // Setup simplest trap framework.
    trap::init();

    // Startup a kernel thread.
    run_queue::init(cpu_id, dtb_pa);
    trap::start();

    let ctx = run_queue::spawn_task_raw(1, move || {
        // Prepare for user app to startup.
        userboot::init(cpu_id, dtb_pa);

        info!("App kernel-thread is running ..");
        // Load userland app into pgd.
        userboot::load();

        // Note: reschedule to have wander-thread run.
        run_queue::yield_now();

        // Start userland app.
        userboot::start();
        userboot::cleanup();
        info!("App kernel-thread yields itself ..");
        run_queue::yield_now();
    });
    run_queue::activate_task(ctx.clone());

    let ctx = run_queue::spawn_task_raw(2, || {
        info!("Wander kernel-thread is running ..");
        info!("Wander kernel-thread enters infinite waiting period ..");
        loop {
            static mut NEXT_DEADLINE: u64 = 0;
            let now_ns = axhal::time::current_time_nanos();
            let deadline = unsafe { NEXT_DEADLINE };
            if now_ns >= deadline {
                info!("Wander is waiting infinitely .. [{:#x}]", now_ns);
                unsafe {
                    NEXT_DEADLINE = now_ns + PERIODIC_INTERVAL_NANOS;
                }
            }
        }
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
