#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    let msg = "\u{1B}[31m\n[rt_tour_1_1]: earlycon!\n\n\u{1B}[m";
    early_console::write_bytes(msg.as_bytes());

    axlog2::init("debug");
    info!("[rt_tour_1_1]: ...");
    info!("[rt_tour_1_1]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
