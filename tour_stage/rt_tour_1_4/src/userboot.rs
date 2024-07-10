/// userboot
///
/// Start first userland app.

use axhal::mem::PAGE_SIZE_4K;
use page_table::paging::PageTable;
use page_table::paging::MappingFlags;
use core::mem;
use pflash::PayloadHead;
extern crate alloc;
use alloc::str;
const USER_APP_ENTRY: usize = 0x1000;

pub fn load(pgd: &mut PageTable) {
    let result = pflash::load_next(None);
    assert!(result.is_some());
    let (va, size) = result.unwrap();
    info!("Got pflash payload: pos {:#x} size {}", va, size);
    let load_code = unsafe { core::slice::from_raw_parts(va as *const _, size) };

    let flags = MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER;
    pgd.map_region_and_fill(USER_APP_ENTRY.into(), PAGE_SIZE_4K, flags).unwrap();
    info!("Map user page: {:#x} ok!", USER_APP_ENTRY);

    let run_code = unsafe { core::slice::from_raw_parts_mut(USER_APP_ENTRY as *mut u8, size) };
    run_code.copy_from_slice(load_code);

    info!("App code: {:?}", &run_code[0..size]);

    // 获取第二个payload
    let mut offset = mem::size_of::<PayloadHead>()+size;
    offset = (offset + 15) & !15; //将偏移向上取整

    let result = pflash::load_next(Some(offset));
    assert!(result.is_some());

    let (va, size) = result.unwrap();
    info!("Got pflash payload: pos {:#x} size {}", va, size);
    let load_arg = unsafe { core::slice::from_raw_parts(va as *const u8, size) };
    info!("Second Payload args: {:?}",str::from_utf8(load_arg).unwrap());

    pgd.unmap_region_and_free(USER_APP_ENTRY.into(), PAGE_SIZE_4K).unwrap();
}
