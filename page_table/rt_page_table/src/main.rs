#![no_std]
#![no_main]

use core::panic::PanicInfo;
use axhal::mem::phys_to_virt;

#[macro_use]
extern crate axlog2;

#[cfg(target_arch = "riscv64")]
const TEST_ADDRESSES: [usize; 2] = [0x1000_1000, 0x2200_0000];

#[cfg(target_arch = "x86_64")]
const TEST_ADDRESSES: [usize; 1] = [0xfec00000];

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_page_table]: ...");

    page_table::init(cpu_id, dtb_pa);

    // Try to access device space.
    for addr in TEST_ADDRESSES {
        let va = phys_to_virt(addr.into()).as_usize();
        let ptr = va as *const u32;
        unsafe {
            // NOTE: for pflash we got Big-Endian form.
            info!("Try to access dev region [{:#X}], got {:#X}", va, *ptr);
        }
    }

    info!("[rt_page_table]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
