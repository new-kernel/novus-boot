use uefi::prelude::{Boot, Handle, SystemTable};
use crate::fs::root_dir;

pub fn start_kernel(image: Handle, system_table: SystemTable<Boot>) {
    let root = root_dir(system_table.boot_services());
}