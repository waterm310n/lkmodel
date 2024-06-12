#![no_std]
#![no_main]

#[macro_use]
extern crate log;
extern crate alloc;

use alloc::string::ToString;
use core::panic::PanicInfo;
use fstree::FsStruct;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_fstree]: ...");

    fstree::init(cpu_id, dtb_pa);

    let fs = fstree::init_fs();
    let cwd = fs.lock().current_dir().unwrap_or("No CWD!".to_string());
    info!("cwd: {}", cwd);

    info!("[rt_fstree]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
