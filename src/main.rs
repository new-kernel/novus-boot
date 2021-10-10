#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use] extern crate log;
pub extern crate rlibc;

pub mod arch;
pub mod uefi_boot;

