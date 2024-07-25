#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use core::panic::PanicInfo;

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    assert_eq!(cpu_id, 0);

    axlog2::init("debug");
    info!("[rt_axmount]: ... cpuid {}", cpu_id);

    axhal::cpu::init_primary(cpu_id);

    info!("Initialize global memory allocator...");
    axalloc::init();

    info!("Initialize kernel page table...");
    page_table::init();

    /*
    let ctx = Arc::new(taskctx::init_sched_info());
    unsafe {
        let ptr = Arc::into_raw(ctx.clone());
        axhal::cpu::set_current_task_ptr(ptr);
    }
    */

    // Init runq just for using mutex.
    //task::init(cpu_id, dtb_pa);
    //taskctx::init(cpu_id, dtb_pa);
    //run_queue::init(cpu_id, dtb_pa);

    {
        //let mut disk = ramdisk::RamDisk::new(0x10000);
        /*
        let mut alldevs = axdriver::init_drivers();
        let disk = alldevs.block.take_one().unwrap();
        let disk = AxDeviceContainer::from_one(disk);

        let main_fs = axmount::init_filesystems(disk, false);
        let root_dir = axmount::init_rootfs(main_fs);
        let mut fs = FsStruct::new();
        fs.init(root_dir);
        */
        fstree::init(cpu_id, dtb_pa);
        let fs = fstree::init_fs();
        let locked_fs = fs.lock();
        match axfile::api::create_dir("/testcases", &locked_fs) {
            Ok(_) => info!("create /testcases ok!"),
            Err(e) => error!("create /testcases failed {}", e),
        }

        let fname = "/testcases/new-file.txt";
        info!("test create file {:?}:", fname);
        //assert_err!(axfile::api::metadata(fname), NotFound);
        let contents = "create a new file!\n";
        axfile::api::write(fname, contents, &locked_fs).unwrap();

        let ret = axfile::api::read_to_string(fname, &locked_fs).unwrap();
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
