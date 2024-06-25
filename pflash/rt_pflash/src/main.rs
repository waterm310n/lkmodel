#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    pflash::init(cpu_id, dtb_pa);
    info!("[rt_pflash]: ...");

    let result = pflash::load_next(None);
    assert!(result.is_some());
    let (va, size) = result.unwrap();
    info!("payload: pos {:#x} size {}", va, size);

    info!("[rt_pflash]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
