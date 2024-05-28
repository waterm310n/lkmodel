#![cfg_attr(not(test), no_std)]
#![feature(naked_functions)]
#![feature(asm_const)]

mod platform;

use core::panic::PanicInfo;

pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
