/// userboot
///
/// Start first userland app.

use alloc::sync::Arc;
use axhal::mem::PAGE_SIZE_4K;
use axhal::arch::start_thread;
use axhal::trap::TRAPFRAME_SIZE;
use page_table::paging::MappingFlags;
use page_table::paging::pgd_alloc;
use axalloc::global_allocator;
use spinbase::SpinNoIrq;
use axhal::arch::write_page_table_root0;

pub const USER_APP_ENTRY: usize = 0x1000;

pub fn load() {
    let fs = fstree::init_fs();
    let locked_fs = fs.lock();

    let fname = "/sbin/origin.bin";
    let load_code = axfile::api::read(fname, &locked_fs).unwrap();
    let size = load_code.len();
    info!("read origin.bin: size [{}]", size);

    let ctx = taskctx::current_ctx();
    let pgd = ctx.try_pgd().expect("Current task has no pgd!");

    let flags = MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER;
    pgd.lock().map_region_and_fill(USER_APP_ENTRY.into(), PAGE_SIZE_4K, flags).unwrap();
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
    let ctx = taskctx::current_ctx();
    let pgd = ctx.try_pgd().expect("Current task has no pgd!");
    pgd.lock().unmap_region_and_free(USER_APP_ENTRY.into(), PAGE_SIZE_4K).unwrap();
}

pub fn init(cpu_id: usize, dtb_pa: usize) {
    fstree::init(cpu_id, dtb_pa);

    // Alloc new pgd and setup.
    let pgd = Arc::new(SpinNoIrq::new(pgd_alloc()));
    unsafe {
        write_page_table_root0(pgd.lock().root_paddr().into());
    }

    taskctx::init(cpu_id, dtb_pa);
    let mut ctx = taskctx::current_ctx();
    assert!(ctx.pgd.is_none());
    ctx.as_ctx_mut().set_mm(1, pgd);
}
