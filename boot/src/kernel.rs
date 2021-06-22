use core::fmt::Write;
use crate::error::error;
use crate::proto::fs::INIT;
use crate::proto::graphics::gop_init;
use uefi::{Handle, ResultExt};
use uefi::prelude::BootServices;
use uefi::proto::console::gop::{BltOp, BltPixel};
use uefi::proto::console::text::Color;
use uefi::proto::media::file::{File, FileAttribute, FileMode, FileSystemVolumeLabel};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi_services::system_table;

unsafe fn start_kernel() -> ! {
    loop {  }
}

unsafe fn load_kernel(bs: &BootServices) {
    let kernel_file = r"efi/boot/kernel.elf";
    let mut buffer = [0u8; 0x100];

    // Get filesystem and root directory again
    let fs = &mut *bs.locate_protocol::<SimpleFileSystem>()
        .expect_success("Cannot open SimpleFileSystem")
        .get();

    let mut root = fs.open_volume()
        .expect_success("Cannot open volumes");

    let volume_label = fs.open_volume()
        .expect_success("Failed to open volume")
        .get_info::<FileSystemVolumeLabel>(&mut buffer)
        .expect_success("Failed to open volumes")
        .volume_label();

    let kernel_file_handle = root.open(kernel_file, FileMode::Read, FileAttribute::empty())
        .expect_success("Failed to open kernel file");
}

fn fill_screen() {
    let gop = unsafe { gop_init(system_table().as_ref().boot_services()) };

    let fill = BltOp::VideoFill {
        color: BltPixel::new(53, 114, 165),
        dest: (0, 0),
        dims: (1024, 768)
    };

    gop.blt(fill);
}

pub unsafe fn setup(image: Handle) -> ! {
    fill_screen();

    system_table().as_ref().stdout().set_color(Color::White, Color::Cyan);

    // Just so the user can see it
    writeln!(system_table().as_ref().stdout(), "{}{}", "Novus boot v", env!("CARGO_PKG_VERSION"));

    info!("Setting up kernel...");
    system_table().as_ref().boot_services().stall(1_000_000);

    if INIT == false {
        error("Couldn't initialize FAT file system, can't start kernel");
        loop {  }
    } else { info!("FAT fs is ok, loading kernel..."); }

    //let (efi, kenrel_entry) = load_kernel(system_table().as_ref().boot_services());

    start_kernel();
}
