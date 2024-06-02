#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_axlog2]: ...");
    info!("[rt_axlog2]: ok!");
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
