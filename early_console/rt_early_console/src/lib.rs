#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    let msg = "\n[early_console]: Hello, ArceOS!\n";
    early_console::write_bytes(msg.as_bytes());
    axhal::misc::terminate();
}

pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
