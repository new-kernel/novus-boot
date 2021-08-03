#![no_std]
#![feature(asm)]

#[cfg(feature = "bios")]
pub mod bios_boot;

#[cfg(feature = "uefi")]
pub mod uefi_boot;
