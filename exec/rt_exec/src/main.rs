//! Startup process for monolithic kernel.

#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use core::panic::PanicInfo;

/// The main entry point for monolithic kernel startup.
#[cfg_attr(not(test), no_mangle)]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb: usize) {
    init(cpu_id, dtb);
    start(cpu_id, dtb);
    panic!("Never reach here!");
}

pub fn init(cpu_id: usize, dtb: usize) {
    axlog2::init("info");
    exec::init(cpu_id, dtb);
    axtrap::init(cpu_id, dtb);
    let mut ctx = taskctx::current_ctx();
    ctx.as_ctx_mut().init(None, fork::task_entry as usize, 0.into());
}

pub fn start(_cpu_id: usize, _dtb: usize) {
    let filename = "/sbin/init";
    let _ = exec::kernel_execve(filename);

    let sp = task::current().pt_regs_addr();
    axhal::arch::ret_from_fork(sp);
    unreachable!();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    axhal::misc::terminate();
    #[allow(unreachable_code)]
    arch_boot::panic(info)
}
