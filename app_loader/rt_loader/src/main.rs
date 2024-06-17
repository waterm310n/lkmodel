#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]
#[cfg(feature = "axstd")]
use axstd::{println,vm, vec::Vec};



extern crate alloc;
use alloc::string::String;

use memory_addr::{PAGE_SIZE_4K, align_down_4k, align_up_4k};

use elf::abi::PT_LOAD;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use elf::segment::ProgramHeader;
use elf::parse::ParseAt;
use mmap::MAP_FIXED;
use preempt_guard::NoPreempt;

const PAGE_SIZE: usize  = 0x1000;
const PAGE_SHIFT: usize = 12;

// const PFLASH_START: usize = 0x22000000; // 启用分页feature之前使用0x22000000 
const PFLASH_START: usize = 0xffff_ffc0_2200_0000; //启用分页

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let mut pos = PFLASH_START;
    let app_num = parse_literal_hex(pos);
    
    assert_eq!(app_num, 2);
    pos += 8;

    for _ in 0..app_num {
        let size = parse_literal_hex(pos);
        println!("app size: {} {:#X}", size,size);
        pos += 8;

        let code = unsafe {
            core::slice::from_raw_parts(pos as *const u8, size)
        };
        pos += size;
        println!("app pos: {:#X}", pos);

        println!("\n==============");
        let (entry, end) = parse_elf(code);
        println!("App: entry: {:#X},end : {:#X}", entry, end);
        
        // run_app(entry, end);
        return ;
        // thread::spawn(move || {
        //     println!("\n==============");
        //     let (entry, end) = parse_elf(code);
        //     println!("App: entry: {:#X}", entry);

        //     run_app(entry, end);
        // });
    }
}

// Note: Length of literal hex must be 8. 
// 一次看8个字节，并将其转换为usize值
fn parse_literal_hex(pos: usize) -> usize {
    let hex = unsafe { core::slice::from_raw_parts(pos as *const u8, 8) };
    let hex = String::from_utf8(hex.into()).expect("bad hex number.");
    usize::from_str_radix(&hex, 16).expect("NOT hex number.")
}

// 将elfflag转换称mapflag
fn elfflags_to_mapflags(flags: usize) -> usize {
    const PF_X: usize = 1 << 0; // Segment is executable
    const PF_W: usize =	1 << 1; // Segment is writable
    const PF_R: usize = 1 << 2; // Segment is readable

    let mut mapflags = 0;
    if flags & PF_X == PF_X {
        mapflags |= vm::EXECUTE;
    }
    if flags & PF_W == PF_W {
        mapflags |= vm::WRITE;
    }
    if flags & PF_R == PF_R {
        mapflags |= vm::READ;
    }
    mapflags
}

// 解析elf文件，同时返回程序的虚拟入口地址与end？（目前返回的虚拟地址）
fn parse_elf(code: &[u8]) -> (usize,usize){
    // 获取程序的虚拟地址对应的entry
    let file = ElfBytes::<AnyEndian>::minimal_parse(code).unwrap();
    println!("e_entry: {:#X}", file.ehdr.e_entry);

    let phdrs: Vec<ProgramHeader> = file.segments().unwrap()
        .iter()
        .filter(|phdr|{phdr.p_type == PT_LOAD})
        .collect();

    let mut end = 0;

    let mut elf_bss: usize = 0;
    let mut elf_brk: usize = 0;

    println!("There are {} PT_LOAD segments", phdrs.len());
    for phdr in &phdrs {
        println!("-------------------phdr---------------------");
        // 打印段偏移，虚拟地址，段在文件中的大小，段在内存中的大小（比段在文件中的大小多的部分用0填充）
        println!(
            "offset: {:#X}, v_addr: {:#X}, filesz:{:#X}, memsz: {:#X}",
            phdr.p_offset, phdr.p_vaddr, phdr.p_filesz, phdr.p_memsz
        );

        // 计算文件在内存中需要占用的页数
        let va_end = align_up_4k((phdr.p_vaddr + phdr.p_memsz) as usize);
        let va = align_down_4k(phdr.p_vaddr as usize);
        let num_pages = (va_end - va) >> PAGE_SHIFT;
        println!("va: {:#X} => va_end:{:#X},num_pages:{:#X}",va,va_end,num_pages);

        // 将elfflags转换为mapflags
        let flags = elfflags_to_mapflags(phdr.p_flags as usize);
        println!("flags: {:#X} => {:#X}", phdr.p_flags, flags);
        
        // 使用虚拟内存分配器进行内存分配
        let pa = vm::alloc_pages(num_pages, PAGE_SIZE_4K);
        println!("va: {:#x} ,va_end: {:#x},pa: {:#x} num_pages {}", va, va_end,pa, num_pages);
        
        // Whatever we need vm::WRITE for initialize segment.
        // Fix it in future.
        vm::map_region(va, pa, num_pages << PAGE_SHIFT, flags|vm::WRITE);

        // 将数据从文件复制到内存中
        let fdata = file.segment_data(&phdr).unwrap();
        let mdata = unsafe {
            core::slice::from_raw_parts_mut(phdr.p_vaddr as *mut u8, phdr.p_filesz as usize)
        };
        mdata.copy_from_slice(fdata);
        println!("mdata: {:#x}", mdata.len());

        // 对于数据段中的bss段，将其清空
        if phdr.p_memsz != phdr.p_filesz { 
            let edata = unsafe {
                core::slice::from_raw_parts_mut((phdr.p_vaddr+phdr.p_filesz) as *mut u8, (phdr.p_memsz - phdr.p_filesz) as usize)
            };
            edata.fill(0);
            println!("清空bss段大小 edata_sz: {:#x}", edata.len());
        }
        
        if end < va_end {
            end = va_end;
        }
        println!("--------------------------------------------");
    }
    (file.ehdr.e_entry as usize, end)
}

// fn run_app(entry: usize, end: usize) {
    
//     // 分配一页作为用户栈?
//     const TASK_SIZE: usize = 0x40_0000_0000;
//     let pa = vm::alloc_pages(1, PAGE_SIZE_4K);
//     let va = TASK_SIZE - PAGE_SIZE_4K;
//     println!("va: {:#x} pa: {:#x}", va, pa);
//     vm::map_region(va, pa, PAGE_SIZE_4K, vm::READ | vm::WRITE);
//     let sp = TASK_SIZE - 32;
//     let stack = unsafe {
//         core::slice::from_raw_parts_mut(
//             sp as *mut usize, 4
//         )
//     };
    
//     stack[0] = 0;
//     stack[1] = TASK_SIZE - 16;
//     stack[2] = 0;
//     stack[3] = 0;

//     // 在应用启动前创建
//     let mut task = task::current();
//     {
//         let _ = NoPreempt::new();
//         task.as_task_mut().alloc_mm();
//     }

//     println!("set brk...");
//     vm::set_brk(end);

//     let pa = vm::alloc_pages(4, PAGE_SIZE_4K);
//     vm::map_region(end, pa, 4*PAGE_SIZE_4K, vm::READ | vm::WRITE);
//     // println!("### app end: {:#X}; {:#X}", end, vm::get_brk());

//     setup_zero_page();

//     println!("Execute app ... at entry {:#x}",entry);

//     // execute app
    
//     unsafe { core::arch::asm!("
//         jalr t0
//         ",
//         in("t0") entry,
//     )};

// }

// fn setup_zero_page() {
//     // 分配一页用作Zero
//     let pa = vm::alloc_pages(1, PAGE_SIZE_4K);
//     vm::map_region(0x0, pa, PAGE_SIZE_4K, vm::READ);
// }