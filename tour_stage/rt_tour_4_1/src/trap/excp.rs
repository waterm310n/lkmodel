pub fn handle_breakpoint(sepc: &mut usize) {
    info!("Exception(Breakpoint) @ {:#x} ", sepc);
    *sepc += 2
}

pub fn init() {
}
