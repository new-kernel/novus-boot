use core::fmt::Write;
use uefi::Handle;
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub extern "C" fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> ! {
    uefi_services::init(&system_table);

    let bootloader_version = env!("CARGO_PKG_VERSION");

    writeln!(system_table.stdout(), "{}{}", "Booting with Novus Boot v{}", bootloader_version);

    loop {  }
}
