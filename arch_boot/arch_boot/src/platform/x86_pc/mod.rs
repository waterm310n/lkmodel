mod boot;

unsafe extern "C" fn rust_entry(magic: usize, mbi: usize) {
    // TODO: handle multiboot info
    if magic == self::boot::MULTIBOOT_BOOTLOADER_MAGIC {
        super::clear_bss();
        runtime_main(current_cpu_id(), mbi);
    }
    unreachable!();
}

fn current_cpu_id() -> usize {
    match raw_cpuid::CpuId::new().get_feature_info() {
        Some(finfo) => finfo.initial_local_apic_id() as usize,
        None => 0,
    }
}

extern "Rust" {
    fn runtime_main(magic: usize, mbi: usize);
}
