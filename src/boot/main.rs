use uefi::Handle;
use uefi::prelude::{Boot, SystemTable};

#[no_mangle]
pub extern "C" fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> ! {
    uefi_services::init(&mut system_table);

    let stdout = system_table.stdout();
    stdout.clear();

    writeln!(stdout, "{}", "UEFI services initialized\nScreen cleared");

    loop {  }
}
