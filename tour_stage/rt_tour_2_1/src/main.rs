#![no_std]
#![no_main]
#![feature(asm_const)]

#[macro_use]
extern crate axlog2;
extern crate alloc;

mod userboot;
mod trap;

use core::panic::PanicInfo;
use alloc::sync::Arc;
use axhal::arch::write_page_table_root0;
use page_table::paging::pgd_alloc;
use spinbase::SpinNoIrq;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_1_6]: ...");

    // Setup simplest trap framework.
    trap::init();

    // Prepare for user app to startup.
    userboot::init(cpu_id, dtb_pa);

    // Alloc new pgd and setup.
    let pgd = Arc::new(SpinNoIrq::new(pgd_alloc()));
    unsafe {
        write_page_table_root0(pgd.lock().root_paddr().into());
    }

    taskctx::init(cpu_id, dtb_pa);
    let mut ctx = taskctx::current_ctx();
    assert!(ctx.pgd.is_none());
    ctx.as_ctx_mut().set_mm(1, pgd);

    // Load userland app into pgd.
    userboot::load();

    // Load userland app into pgd.
    userboot::start();

    userboot::cleanup();

    unreachable!();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
