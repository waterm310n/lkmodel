//! Startup process for monolithic kernel.

#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use axerrno::{LinuxError, LinuxResult};
use axhal::mem::{memory_regions, phys_to_virt};
use axtype::DtbInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use fork::{user_mode_thread, CloneFlags};
use core::panic::PanicInfo;

static INITED_CPUS: AtomicUsize = AtomicUsize::new(0);

fn is_init_ok() -> bool {
    INITED_CPUS.load(Ordering::Acquire) == axconfig::SMP
}

const LOGO: &str = r#"
       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"
"#;

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

pub fn start(_cpu_id: usize, dtb: usize) {
    let filename = "/sbin/init";
    exec::kernel_execve(filename);

    let sp = task::current().pt_regs_addr();
    axhal::arch::ret_from_fork(sp);
    unreachable!();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}
