#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    loop { asm!("hlt") }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
