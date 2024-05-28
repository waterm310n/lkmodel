#![no_std]

#[macro_use]
extern crate axlog2;
extern crate alloc;
use alloc::sync::Arc;

use core::panic::PanicInfo;
use axtype::{align_up_4k, align_down_4k, phys_to_virt, virt_to_phys};
use driver_block::{ramdisk, BlockDriverOps};
use axdriver::{prelude::*, AxDeviceContainer};
use axhal::mem::memory_regions;
use fstree::FsStruct;
use axfile::api::File;
use axfile::api::OpenOptions;
use axio::{Seek, SeekFrom, Read};

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, _dtb_pa: usize) {
    assert_eq!(cpu_id, 0);

    axlog2::init();
    axlog2::set_max_level("debug");
    info!("[rt_axmount]: ... cpuid {}", cpu_id);

    axhal::cpu::init_primary(cpu_id);

    let start = align_up_4k(virt_to_phys(_ekernel as usize));
    let end = align_down_4k(axconfig::PHYS_MEMORY_END);
    axalloc::global_init(phys_to_virt(start), end - start);

    info!("Initialize kernel page table...");
    remap_kernel_memory().expect("remap kernel memoy failed");

    let ctx = Arc::new(taskctx::init_sched_info());
    unsafe {
        let ptr = Arc::into_raw(ctx.clone());
        axhal::cpu::set_current_task_ptr(ptr);
    }

    // Init runq just for using mutex.
    let idle = task::init();
    run_queue::init(idle);

    {
        //let mut disk = ramdisk::RamDisk::new(0x10000);
        let mut alldevs = axdriver::init_drivers();
        let mut disk = alldevs.block.take_one().unwrap();
        let mut disk = AxDeviceContainer::from_one(disk);

        let main_fs = axmount::init_filesystems(disk, false);
        let root_dir = axmount::init_rootfs(main_fs);
        let mut fs = FsStruct::new();
        fs.init(root_dir);

        let filename = "/lib64/ld-linux-x86-64.so.2";
        let mut file = File::open(filename, &fs).unwrap();

        let mut buf: [u8; 512] = [0u8; 512];
        let offset = 0x2200;
        let _ = file.seek(SeekFrom::Start(offset as u64));
        let ret = file.read(&mut buf).unwrap();
        info!("*** READ: [{}]", ret);
        info!("*** READ: {:#X} {:#X} {:#X} {:#X}", buf[0x4a], buf[0x4b], buf[0x4c], buf[0x4d]);
    }

    info!("[rt_axmount]: ok!");
    axhal::misc::terminate();
}

fn remap_kernel_memory() -> Result<(), axhal::paging::PagingError> {
    use axhal::paging::PageTable;
    use axhal::paging::{reuse_page_table_root, setup_page_table_root};
    use axhal::mem::phys_to_virt;

    let mut kernel_page_table = PageTable::try_new()?;
    for r in memory_regions() {
        kernel_page_table.map_region(
            phys_to_virt(r.paddr),
            r.paddr,
            r.size,
            r.flags.into(),
            true,
        )?;
    }
    setup_page_table_root(kernel_page_table);

    Ok(())
}

pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}

extern "C" {
    fn _ekernel();
}
