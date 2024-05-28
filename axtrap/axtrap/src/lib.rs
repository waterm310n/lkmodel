#![cfg_attr(not(test), no_std)]
#![feature(asm_const)]

#[macro_use]
extern crate log;

mod arch;
pub mod irq;
mod platform;

pub fn init() {
    arch::init_trap();
}
