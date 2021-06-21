use crate::proto::graphics::gop_init;
use uefi::Handle;
use uefi::proto::console::gop::{BltOp, BltPixel};
use uefi::proto::console::text::Color;
use uefi_services::system_table;

unsafe fn start_kernel() -> ! {
    loop {  }
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

    info!("Setting up kernel...");
    start_kernel()
}
