/// userboot
///
/// Start first userland app.

use axhal::mem::PAGE_SIZE_4K;
use axhal::arch::start_thread;
use axhal::trap::TRAPFRAME_SIZE;
use page_table::paging::MappingFlags;
use axalloc::global_allocator;

const USER_APP_ENTRY: usize = 0x1000;

pub fn load() {
    let result = pflash::load_next(None);
    assert!(result.is_some());
    let (va, size) = result.unwrap();
    info!("Got pflash payload: pos {:#x} size {}", va, size);
    let load_code = unsafe { core::slice::from_raw_parts(va as *const _, size) };

    let ctx = taskctx::current_ctx();
    let pgd = ctx.try_pgd().expect("Current task has no pgd!");

    let flags = MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER;
    pgd.lock().map_region_and_fill(USER_APP_ENTRY.into(), PAGE_SIZE_4K, flags).unwrap();
    info!("Map user page: {:#x} ok!", USER_APP_ENTRY);

    let run_code = unsafe { core::slice::from_raw_parts_mut(USER_APP_ENTRY as *mut u8, size) };
    run_code.copy_from_slice(load_code);

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
    pflash::init(cpu_id, dtb_pa);
}
