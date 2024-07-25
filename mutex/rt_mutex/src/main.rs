#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use core::panic::PanicInfo;
use axtype::{align_up_4k, align_down_4k, phys_to_virt, virt_to_phys};
use mutex::Mutex;

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    axhal::cpu::init_primary(cpu_id);

    axlog2::init("debug");
    info!("[rt_mutex]: ...");

    let start = align_up_4k(virt_to_phys(_ekernel as usize));
    let end = align_down_4k(axconfig::PHYS_MEMORY_END);
    axalloc::global_init(phys_to_virt(start), end - start);

    run_queue::init(cpu_id, dtb_pa);

    {
        let mutex: Mutex<u32> = Mutex::new(0);
        // Todo: do some tests according tests below.
        info!("{}", *mutex.lock());
    }

    info!("[rt_mutex]: ok!");
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

/*
mod tests {
    use crate::Mutex;
    use axtask as thread;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn may_interrupt() {
        // simulate interrupts
        if rand::random::<u32>() % 3 == 0 {
            thread::yield_now();
        }
    }

    #[test]
    fn lots_and_lots() {
        INIT.call_once(thread::init_scheduler);

        const NUM_TASKS: u32 = 10;
        const NUM_ITERS: u32 = 10_000;
        static M: Mutex<u32> = Mutex::new(0);

        fn inc(delta: u32) {
            for _ in 0..NUM_ITERS {
                let mut val = M.lock();
                *val += delta;
                may_interrupt();
                drop(val);
                may_interrupt();
            }
        }

        for _ in 0..NUM_TASKS {
            thread::spawn(|| inc(1));
            thread::spawn(|| inc(2));
        }

        println!("spawn OK");
        loop {
            let val = M.lock();
            if *val == NUM_ITERS * NUM_TASKS * 3 {
                break;
            }
            may_interrupt();
            drop(val);
            may_interrupt();
        }

        assert_eq!(*M.lock(), NUM_ITERS * NUM_TASKS * 3);
        println!("Mutex test OK");
    }
}
*/
