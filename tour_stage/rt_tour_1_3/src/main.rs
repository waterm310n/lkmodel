#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;

use core::panic::PanicInfo;
use core::{mem, str, slice};
use axhal::mem::{phys_to_virt, virt_to_phys, PAGE_SIZE_4K};
use axhal::arch::write_page_table_root0;
use page_table::paging::pgd_alloc;
use page_table::paging::MappingFlags;

const PFLASH_START: usize = 0x2200_0000;
const USER_APP_ENTRY: usize = 0x1000;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_1_3]: ...");

    page_table::init(cpu_id, dtb_pa);

    // Alloc new pgd and setup.
    let mut pgd = pgd_alloc();
    unsafe {
        write_page_table_root0(pgd.root_paddr().into());
    }

    // Makesure that we can access pflash region.
    let va = phys_to_virt(PFLASH_START.into()).as_usize();
    let ptr = va as *const u32;
    unsafe {
        info!("Try to access dev region [{:#X}], got {:#X}", va, *ptr);
        let magic = mem::transmute::<u32, [u8; 4]>(*ptr);
        info!("Got pflash magic: {}", str::from_utf8(&magic).unwrap());
    }

    // Makesure that we can map a user-page and read/write/execute.
    let va: usize = axalloc::global_allocator().alloc_pages(1, PAGE_SIZE_4K).unwrap();
    let pa = virt_to_phys(va.into());
    let flags = MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER;
    pgd.map_region(USER_APP_ENTRY.into(), pa.into(), PAGE_SIZE_4K, flags, true).unwrap();
    info!("Map user page: {:#x} ok!", USER_APP_ENTRY);

    let dwords = unsafe {
        slice::from_raw_parts_mut(USER_APP_ENTRY as *mut u64, PAGE_SIZE_4K/8)
    };
    for dw in dwords.iter_mut() {
        *dw = 0xAABBCCDD;
    }
    for dw in dwords {
        assert_eq!(*dw, 0xAABBCCDD);
    }
    let _ = pgd.unmap_region(USER_APP_ENTRY.into(), PAGE_SIZE_4K);

    info!("[rt_tour_1_3]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
