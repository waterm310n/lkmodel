//! Startup process for monolithic kernel.

#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use core::panic::PanicInfo;
use alloc::vec;
use alloc::borrow::ToOwned;

/// The main entry point for monolithic kernel startup.
#[cfg_attr(not(test), no_mangle)]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb: usize) {
    init(cpu_id, dtb);
    start(cpu_id, dtb);
    panic!("Never reach here!");
}

pub fn init(cpu_id: usize, dtb: usize) {
    axlog2::init("error");
    exec::init(cpu_id, dtb);
    axtrap::init(cpu_id, dtb);
}

pub fn start(_cpu_id: usize, _dtb: usize) {
    let init_cmd = env!("AX_INIT_CMD");
    if init_cmd.len() == 0 {
        panic!("No init_cmd!");
    }

    console_on_rootfs();

    let _ = exec::kernel_execve(init_cmd, vec![init_cmd.to_owned()], vec![]);

    let sp = task::current().pt_regs_addr();
    axhal::arch::ret_from_fork(sp);
    unreachable!();
}

fn console_on_rootfs() {
    use alloc::sync::Arc;
    use axfile::fops::File;
    use axfile::fops::OpenOptions;
    use mutex::Mutex;

    let mut opts = OpenOptions::new();
    opts.read(true);
    opts.write(true);

    let current = task::current();
    let fs = current.fs.lock();
    let console = File::open("/dev/console", &opts, &fs)
        .expect("bad /dev/console");
    let console = Arc::new(Mutex::new(console));

    let stdin = current.filetable.lock().insert(console.clone());
    error!("Register stdin: fd[{}]", stdin);
    let stdout = current.filetable.lock().insert(console.clone());
    error!("Register stdout: fd[{}]", stdout);
    let stderr = current.filetable.lock().insert(console.clone());
    error!("Register stderr: fd[{}]", stderr);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    axhal::misc::terminate();
    #[allow(unreachable_code)]
    arch_boot::panic(info)
}
