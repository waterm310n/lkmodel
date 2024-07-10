#![no_std]

use core::{mem, slice};
use axtype::{phys_to_virt, is_aligned};

const PFLASH_START: usize = 0x2200_0000;
const MAGIC: u32 = 0x64_6C_66_70;
const VERSION: u32 = 0x01;

pub struct PayloadHead {
    _magic: u32,
    _version: u32,
    _size: u32,
    _pad: u32, // 如果_pad的值为1，则表示参数，否则表示应用
}

/// Load next payload from offset in pflash.
/// offset:
///   - Some(offset): next payload head position.
///   - None: from payload start position.
pub fn load_next(offset: Option<usize>) -> Option<(usize, usize)> {
    let offset = offset.unwrap_or(0);
    let va = phys_to_virt(PFLASH_START + offset);
    assert!(is_aligned(va, 16));
    let data = va as *const u32;
    let data = unsafe {
        slice::from_raw_parts(data, mem::size_of::<PayloadHead>())
    };
    assert_eq!(data[0], MAGIC);
    assert_eq!(data[1].to_be(), VERSION);

    Some((va + mem::size_of::<PayloadHead>(), data[2].to_be() as usize))
}

pub fn init(cpu_id: usize, dtb_pa: usize) {
    axconfig::init_once!();
    page_table::init(cpu_id, dtb_pa);
}
