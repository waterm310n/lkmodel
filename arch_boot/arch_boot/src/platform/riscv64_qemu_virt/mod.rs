mod boot;

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    super::clear_bss();
    // Todo: remove it in future.
    // We need to enable sum only when necessary.
    riscv::register::sstatus::set_sum();

    // 因为ABI的符号修改了，此处也进行修改
    main(cpu_id, dtb);
}

extern "Rust" {
    // 此处我将runtime_main修改为了main，因为通常函数入口是main，
    // 所以我将此处的ABI定义修改为了main
    fn main(cpu_id: usize, dtb: usize);
}
