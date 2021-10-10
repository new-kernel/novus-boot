use uefi::Handle;
use uefi::table::{Boot, SystemTable};
use novus_boot::KernelBootInfo;

#[no_mangle]
pub extern "C" fn efi_main(image: Handle, mut sys_table: SystemTable<Boot>) -> ! {
    if uefi_services::init(&mut sys_table).is_ok() {
        info!("UEFI initialized successfully");
    } else { panic!("Couldn't initialized UEFI services"); }

    let boot_info = KernelBootInfo { bootloader_name: "Novus Boot" }'

    loop {  }
}
