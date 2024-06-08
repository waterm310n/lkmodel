#![no_std]

use core::{mem::align_of, mem::size_of_val};

pub struct UserStack {
    _base: usize,
    sp: usize,
    ptr: usize,
}

impl UserStack {
    pub fn new(base: usize, ptr: usize) -> Self {
        Self {
            _base: base,
            sp: base,
            ptr,
        }
    }

    pub fn get_sp(&self) -> usize {
        self.sp
    }

    pub fn push<T: Copy>(&mut self, data: &[T]) {
        let origin = self.sp;
        self.sp -= size_of_val(data);
        self.sp -= self.sp % align_of::<T>();
        self.ptr -= origin - self.sp;
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr as *mut T, data.len())
                .copy_from_slice(data);
        }
    }
    pub fn push_str(&mut self, str: &str) -> usize {
        self.push(&[b'\0']);
        self.push(str.as_bytes());
        self.sp
    }
}

pub fn init() {
    axconfig::init_once!();
}
