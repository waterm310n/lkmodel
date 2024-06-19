#![no_std]

#[macro_use]
extern crate log;
extern crate alloc;
use alloc::vec;
use alloc::string::String;

use axerrno::LinuxResult;
use axhal::arch::start_thread;
use mmap::{MAP_ANONYMOUS, MAP_FIXED, PROT_READ};
use axtype::{PAGE_SIZE, get_user_str_vec};

pub fn kernel_execve(filename: &str) -> LinuxResult {
    info!("kernel_execve... {}", filename);

    task::alloc_mm();
    let _ = setup_zero_page();

    let args = vec![filename.into()];
    let (entry, sp) = bprm_loader::execve(filename, 0, args)?;

    info!("start thread...");
    start_thread(task::current().pt_regs_addr(), entry, sp);
    Ok(())
}

#[allow(unused)]
fn setup_zero_page() -> LinuxResult {
    info!("setup_zero_page ...");
    mmap::_mmap(0x0, PAGE_SIZE, PROT_READ, MAP_FIXED | MAP_ANONYMOUS, None, 0)?;
    Ok(())
}

pub fn execve(path: &str, argv: usize, envp: usize) -> usize {
    info!("execve: {}", path);

    let mut args = get_user_str_vec(argv);
    assert!(args.len() > 0);
    args[0] = String::from(path);
    for arg in &args {
        info!("arg: {}", arg);
    }
    let envp = get_user_str_vec(envp);
    for env in &envp {
        info!("env: {}", env);
    }
    assert!(envp.len() <= 1);

    task::alloc_mm();

    // TODO: Move it into kernel_init().
    let _ = setup_zero_page();
    let (entry, sp) = bprm_loader::execve(path, 0, args).expect("exec error!");

    info!("start thread...");
    start_thread(task::current().pt_regs_addr(), entry, sp);
    0
}

pub fn init(cpu_id: usize, dtb_pa: usize) {
    axconfig::init_once!();

    axlog2::init(option_env!("AX_LOG").unwrap_or(""));
    axhal::arch_init_early(cpu_id);
    axalloc::init();
    page_table::init();
    axhal::platform_init();
    task::init(cpu_id, dtb_pa);
    bprm_loader::init(cpu_id, dtb_pa);
}
