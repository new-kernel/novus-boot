use uefi::Handle;
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub extern "C" fn efi_main(image: Handle, mut sys_table: SystemTable<Boot>) -> ! {
    if uefi_services::init(&mut sys_table).is_ok() {
        info!("UEFI initialized successfully");
    } else { panic!("Couldn't initialized UEFI services"); }

    loop {  }
}
