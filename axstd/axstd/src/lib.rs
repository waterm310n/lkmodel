
#![cfg_attr(all(not(test), not(doc)), no_std)]
#![feature(doc_cfg)]
#![feature(doc_auto_cfg)]

// #[macro_use]
// mod macros;

// pub mod io;
pub use axlog2::ax_print as print;
pub use axlog2::ax_println as println;

use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}