#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;
use alloc::string::String;

use core::slice;
use core::panic::PanicInfo;
use axalloc::global_allocator;
use axhal::mem::PAGE_SIZE_4K;

#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_1_2]: ...");

    axalloc::init();

    let s = String::from("Hello, axalloc!");
    info!("Alloc string: {}", s);
    info!("Before alloc page: Current Avaliable Bytes {}",global_allocator().available_bytes());
    info!("Before alloc page: Current Avaliable Bytes {}",global_allocator().available_pages());
    let va = global_allocator().alloc_pages(1, PAGE_SIZE_4K).unwrap();
    info!("Alloc page: {:#x}", va);
    info!("after alloc page: Current Avaliable Bytes {}",global_allocator().available_bytes());
    info!("after alloc page: Current Avaliable Bytes {}",global_allocator().available_pages());

    let dwords = unsafe {
        slice::from_raw_parts_mut(va as *mut u64, PAGE_SIZE_4K/8)
    };
    for dw in dwords.iter_mut() {
        *dw = 0xAABBCCDD;
    }
    for dw in dwords {
        assert_eq!(*dw, 0xAABBCCDD);
    }
    global_allocator().dealloc_pages(va, 1);
    info!("Dealloc page: {:#x}", va);

    info!("[rt_tour_1_2]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
