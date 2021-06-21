use crate::error::error;
use uefi::ResultExt;
use uefi::prelude::BootServices;
use uefi::proto::console::gop::{GraphicsOutput, ModeInfo};
use uefi_services::system_table;

pub(crate) fn get_mode() -> ModeInfo {
    let gop = unsafe { gop_init(system_table().as_ref().boot_services()) };

    return gop.current_mode_info();
}

fn set_mode(gop: &mut GraphicsOutput) {
    let mode = gop
        .modes()
        .map(|mode| mode.expect("Warnings encountered while querying mode"))
        .find(|mode| {
            let info = mode.info();
            info.resolution() == (1024, 768)
        })
        .unwrap();

    gop.set_mode(&mode)
        .expect_success("Failed to set graphics mode");
}

pub(crate) fn gop_init(bt: &BootServices) -> &mut GraphicsOutput {
    if let Ok(gop) = bt.locate_protocol::<GraphicsOutput>() {
        let gop = gop.expect("Couldn't initialize GOP");
        let gop = unsafe { &mut *gop.get() };
        set_mode(gop);
        return gop;
    } else {
        error("Failed to initialized graphics protocall");
        return gop_init(bt);
    }
    return gop_init(bt);
}