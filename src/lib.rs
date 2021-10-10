#![no_std]


pub struct KernelBootInfo {
    pub bootloader_name: &'static str,
}

impl KernelBootInfo {
    pub fn new(name: &str) -> Self {
        return KernelBootInfo { bootloader_name: name };
    }
}
