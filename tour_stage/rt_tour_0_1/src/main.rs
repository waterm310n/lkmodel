#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    sbi_rt::legacy::console_putchar('R' as usize);
    sbi_rt::legacy::console_putchar('U' as usize);
    sbi_rt::legacy::console_putchar('S' as usize);
    sbi_rt::legacy::console_putchar('T' as usize);
    sbi_rt::legacy::console_putchar('\n' as usize);
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
