mod boot;
mod desc;

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    super::clear_bss();
    runtime_main(cpu_id, dtb);
}

extern "Rust" {
    fn runtime_main(cpu_id: usize, dtb: usize);
}
