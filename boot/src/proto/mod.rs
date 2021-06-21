use uefi::table::{Boot, SystemTable};

pub(crate) mod graphics;
pub(crate) mod text;

pub fn proto_init(st: SystemTable<Boot>) {
    text::text_init(st.stdout());
    graphics::gop_init(st.boot_services());
}
