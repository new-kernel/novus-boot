use core::fmt::Write;
use crate::error::error;
use crate::{kernel, proto};
use uefi::{Handle, ResultExt};
use uefi::proto::console::text::Color;
use uefi::table::{Boot, SystemTable};
use uefi_services::system_table;

unsafe fn print_info() {
    let stdout = system_table().as_ref().stdout();

    writeln!(stdout, "");
    writeln!(stdout, "{}{:?}", "    Graphics mode: ", proto::graphics::get_mode());
    writeln!(stdout, "{}{:?}", "    Text mode: ", stdout.current_mode().unwrap_success());
}

#[no_mangle]
pub extern "C" fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> ! {
    uefi_services::init(&system_table);

    system_table.stdout().reset(false).expect_success("Failed to reset stdout");
    system_table.stdout().set_color(Color::White, Color::Black);

    let bootloader_version = env!("CARGO_PKG_VERSION");

    writeln!(system_table.stdout(), "{}{}", "Booting with Novus Boot v", bootloader_version);

    proto::proto_init(system_table);

    info!("Protocall setup finished");
    unsafe { print_info(); }

    unsafe { kernel::setup(handle); }
}
