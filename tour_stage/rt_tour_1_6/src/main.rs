#![no_std]
#![no_main]
#![feature(asm_const)]

#[macro_use]
extern crate axlog2;

mod userboot;
mod trap;

use core::panic::PanicInfo;
use axhal::arch::write_page_table_root0;
use page_table::paging::pgd_alloc;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_1_6]: ...");

    // Setup simplest trap framework.
    trap::init();

    // Prepare for user app to startup.
    userboot::init(cpu_id, dtb_pa);

    // Alloc new pgd and setup.
    let mut pgd = pgd_alloc();
    unsafe {
        write_page_table_root0(pgd.root_paddr().into());
    }

    // Load userland app into pgd.
    userboot::load(&mut pgd);

    // Load userland app into pgd.
    userboot::start();

    userboot::cleanup(&mut pgd);

    unreachable!();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
