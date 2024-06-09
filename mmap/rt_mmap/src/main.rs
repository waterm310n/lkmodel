#![no_std]
#![no_main]

use core::panic::PanicInfo;
use mm::{MmStruct, VmAreaStruct};

#[macro_use]
extern crate axlog2;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, _dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_mmap]: ...");

    axhal::arch_init_early(cpu_id);
    axalloc::init();
    page_table::init();

    let mut mm = MmStruct::new();

    let va = 0;
    let vma = VmAreaStruct::new(va, 4096, 0, None, 0);
    mm.vmas.insert(va, vma);

    info!("[rt_mmap]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
