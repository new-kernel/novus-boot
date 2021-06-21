use uefi::table::{Boot, SystemTable};

pub static mut INIT: bool = true;

pub unsafe fn fs_init(st: SystemTable<Boot>) {

}
