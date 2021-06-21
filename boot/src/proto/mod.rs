use uefi::table::{Boot, SystemTable};

pub(crate) mod fs;
pub(crate) mod graphics;
pub(crate) mod text;

pub fn proto_init(st: SystemTable<Boot>) {
    text::text_init(st.stdout());
    graphics::gop_init(st.boot_services());
    unsafe { fs::fs_init(st); }
}
