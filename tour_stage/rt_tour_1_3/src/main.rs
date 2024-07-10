#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;

use core::panic::PanicInfo;
use core::{mem, str, slice};
use axhal::mem::{phys_to_virt, PAGE_SIZE_4K};
use axhal::arch::write_page_table_root0;
use page_table::paging::pgd_alloc;
use page_table::paging::MappingFlags;

const PFLASH_START: usize = 0x2200_0000;
const USER_APP_ENTRY: usize = 0x1000;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_tour_1_3]: ...");

    /*
        简单说一下我理解页表初始化中主要的流程。
        
        首先初始化全局分配器，全局分配器的地址在高地址的bss段，
        全局分配器初始的时候使用的区域是memory_region中最大的free区域,这一部分传给全局分配器的是虚拟地址
        因此全局分配器内部使用的都是虚拟地址，它返回的也是虚拟地址，也就是加上了0xfff..c的地址。
        此外在初始过程中还有可能有一些free的区域，这部分区域全局分配器添加的时候是直接分配给了balloc管理，
        我的理解是因为对于palloc，这部分地址可能不连续，因此palloc的位图来说，会很浪费空间而且比较麻烦。
        因为它是将地址线性地映射为一个线段树管理的位图。
        而对于balloc，使用默认的tlsf时，空闲的地址块是通过链表的形式组织的。因此很容易添加。

        接下来就是创建根页表，他会为memory_region中的地址创建页表，在实际中，他总共创建了高地址空间起始的3G内存空间

        在后续的pgd_alloc()时，每创建一个新的页表，都会从根页表中复制，不过复制的时候只会复制高地址空间中的页表项。
        我的理解是这样可以保证能够正常的访问到axalloc的全局分配器，也就是多个页表可以共享同一个axalloc的全局分配器。
        同时低地址空间页表不同，可以实现用户空间隔离
    */
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

    // 仿照PFlash对头信息magic的输出方式，输出virtio_mmio第一个区域开始的4个字节，该区域的起始物理地址0x10001000。
    let va = phys_to_virt(0x10001000usize.into()).as_usize();
    let ptr = va as *const u32;
    unsafe {
        info!("Try to access dev region [{:#X}], got {:#X}", va, *ptr);
        let magic = mem::transmute::<u32, [u8; 4]>(*ptr);
        info!("Got mmio magic: {}", str::from_utf8(&magic).unwrap());
    }
    // Makesure that we can map a user-page and read/write/execute.
    let flags = MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER;
    pgd.map_region_and_fill(USER_APP_ENTRY.into(), PAGE_SIZE_4K, flags).unwrap();
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
    pgd.unmap_region_and_free(USER_APP_ENTRY.into(), PAGE_SIZE_4K).unwrap();

    info!("[rt_tour_1_3]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    arch_boot::panic(info)
}
