#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;
use alloc::sync::Arc;

use core::panic::PanicInfo;
use axdriver::{AxDeviceContainer};
use fstree::FsStruct;

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, _dtb_pa: usize) {
    assert_eq!(cpu_id, 0);

    axlog2::init("debug");
    info!("[rt_axmount]: ... cpuid {}", cpu_id);

    axhal::cpu::init_primary(cpu_id);

    info!("Initialize global memory allocator...");
    axalloc::init();

    info!("Initialize kernel page table...");
    page_table::init();

    let ctx = Arc::new(taskctx::init_sched_info());
    unsafe {
        let ptr = Arc::into_raw(ctx.clone());
        axhal::cpu::set_current_task_ptr(ptr);
    }

    // Init runq just for using mutex.
    task::init();

    {
        //let mut disk = ramdisk::RamDisk::new(0x10000);
        let mut alldevs = axdriver::init_drivers();
        let disk = alldevs.block.take_one().unwrap();
        let disk = AxDeviceContainer::from_one(disk);

        let main_fs = axmount::init_filesystems(disk, false);
        let root_dir = axmount::init_rootfs(main_fs);
        let mut fs = FsStruct::new();
        fs.init(root_dir);
        match axfile::api::create_dir("/testcases", &fs) {
            Ok(_) => info!("create /testcases ok!"),
            Err(e) => error!("create /testcases failed {}", e),
        }

        let fname = "/testcases/new-file.txt";
        info!("test create file {:?}:", fname);
        //assert_err!(axfile::api::metadata(fname), NotFound);
        let contents = "create a new file!\n";
        axfile::api::write(fname, contents, &fs).unwrap();

        let ret = axfile::api::read_to_string(fname, &fs).unwrap();
        info!("read test file: \"{}\"", ret);
    }

    info!("[rt_axmount]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}

extern "C" {
    fn _ekernel();
}
