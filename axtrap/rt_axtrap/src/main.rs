//! Startup process for monolithic kernel.

#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;

use core::panic::PanicInfo;
use core::arch::asm;

/// The main entry point for monolithic kernel startup.
#[cfg_attr(not(test), no_mangle)]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb: usize) {
    init(cpu_id, dtb);
    start(cpu_id, dtb);

    test_excp();

    // Todo: test syscalls

    // Todo: Replace loop with sleep && exit.
    loop {}
}

fn test_excp() {
    // Todo: test other excp, e.g., #PF
    unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!("int3");
        #[cfg(target_arch = "aarch64")]
        asm!("brk #0");
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        asm!("ebreak");
    }
}

pub fn init(cpu_id: usize, dtb: usize) {
    axtrap::init(cpu_id, dtb);
}

pub fn start(cpu_id: usize, dtb: usize) {
    axtrap::start(cpu_id, dtb);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}
