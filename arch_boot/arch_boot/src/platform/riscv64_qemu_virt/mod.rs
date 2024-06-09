mod boot;

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    super::clear_bss();
    // Todo: remove it in future.
    // We need to enable sum only when necessary.
    riscv::register::sstatus::set_sum();
    
    rust_main(cpu_id, dtb);
}

extern "Rust" {

    fn rust_main(cpu_id: usize, dtb: usize);
}
