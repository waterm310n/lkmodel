#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;

mod userboot;

use core::panic::PanicInfo;
use axhal::arch::write_page_table_root0;
use page_table::paging::pgd_alloc;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    pflash::init(cpu_id, dtb_pa);
    info!("[rt_tour_1_4]: ...");

    // Alloc new pgd and setup.
    let mut pgd = pgd_alloc();
    unsafe {
        write_page_table_root0(pgd.root_paddr().into());
    }

    // Load userland app into pgd.
    userboot::load(&mut pgd);

    info!("[rt_tour_1_4]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
