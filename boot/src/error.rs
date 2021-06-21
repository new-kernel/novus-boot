use uefi::proto::console::text::Color;
use uefi_services::system_table;

pub(crate) fn error(msg: &str) {
    let stdout = unsafe { system_table().as_ref().stdout() };

    stdout.set_color(Color::Red, Color::Black);

    error!("{}", msg);

    stdout.set_color(Color::White, Color::Black);
}
