use core::fmt::Write;
use crate::{fs::file_system_init, gop::gop_init};
use uefi::{Handle, ResultExt};
use uefi::prelude::{Boot, SystemTable};

#[no_mangle]
pub extern "C" fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> ! {
    uefi_services::init(&mut system_table).expect_success("Failed to initialize UEFI");

    let stdout = system_table.stdout();
    stdout.clear();

    writeln!(stdout, "{}", "UEFI services initialized");

    // gop_init(&mut system_table);
    writeln!(stdout, "{}", "GOP initialized");
    writeln!(stdout, "{}", "Starting kernel...");


    loop {  }
}
