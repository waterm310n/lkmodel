/// userboot
///
/// Start first userland app.

use axhal::mem::PAGE_SIZE_4K;
use axhal::arch::start_thread;
use axhal::trap::TRAPFRAME_SIZE;
use axalloc::global_allocator;
use mmap::{PROT_READ, PROT_WRITE, PROT_EXEC, MAP_FIXED};

pub const USER_APP_ENTRY: usize = 0x1000;

pub fn load() {
    let fs = fstree::init_fs();
    let locked_fs = fs.lock();

    let fname = "/sbin/origin.bin";
    let load_code = axfile::api::read(fname, &locked_fs).unwrap();
    let size = load_code.len();
    info!("read origin.bin: size [{}]", size);

    let prot = PROT_READ | PROT_WRITE | PROT_EXEC;
    let _ = mmap::_mmap(USER_APP_ENTRY.into(), PAGE_SIZE_4K, prot, MAP_FIXED, None, 0);
    mmap::faultin_page(USER_APP_ENTRY, 0);
    info!("Map user page: {:#x} ok!", USER_APP_ENTRY);

    let run_code = unsafe { core::slice::from_raw_parts_mut(USER_APP_ENTRY as *mut u8, size) };
    run_code.copy_from_slice(&load_code);

    info!("App code: {:?}", &run_code[0..size]);
}

pub fn start() {
    // Prepare kernel stack
    let ksp = global_allocator().alloc_pages(1, PAGE_SIZE_4K).unwrap();
    info!("Alloc page: {:#x}", ksp);

    let pt_regs = ksp + PAGE_SIZE_4K - TRAPFRAME_SIZE;
    start_thread(pt_regs, USER_APP_ENTRY, 0);
    axhal::arch::ret_from_fork(pt_regs);
}

pub fn cleanup() {
    mmap::munmap(USER_APP_ENTRY.into(), PAGE_SIZE_4K);
}

pub fn init(_cpu_id: usize, _dtb_pa: usize) {
    task::alloc_mm();
}
