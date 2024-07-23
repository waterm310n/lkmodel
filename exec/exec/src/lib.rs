#![no_std]

#[macro_use]
extern crate log;
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

use axerrno::LinuxResult;
use axhal::arch::start_thread;
use axtype::get_user_str_vec;

pub fn kernel_execve(filename: &str, argv: Vec<String>, envp: Vec<String>) -> LinuxResult {
    info!("kernel_execve... {}", filename);

    task::alloc_mm();

    let (entry, sp) = bprm_loader::execve(filename, 0, argv, envp)?;

    info!("start thread... usp {:#x}", sp);
    start_thread(task::current().pt_regs_addr(), entry, sp);
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

    kernel_execve(path, args, envp).expect("exec error!");
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
