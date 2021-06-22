use core::fmt::Write;
use crate::error::error;
use uefi::prelude::BootServices;
use uefi::table::{Boot, SystemTable};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{Directory, File};
use uefi_services::system_table;

pub static mut INIT: bool = true;

pub unsafe fn fs_init(bs: &BootServices) -> Directory {
    if let Ok(sfs) = bs.locate_protocol::<SimpleFileSystem>() {
        let sfs = sfs.expect("Cannot open `SimpleFileSystem` protocol");
        let sfs = unsafe { &mut *sfs.get() };
        let mut buffer = vec![0; 128];
        let mut root = sfs.open_volume().unwrap().unwrap();
        loop {
            let file_info = match root.read_entry(&mut [0; 128]) {
                Ok(completion) => {
                    if let Some(info) = completion.unwrap() {
                        info!("Root directory:\n{:?}", info);
                        info
                    } else {
                        break
                    }
                },
                Err(error) => {
                    let min_size = error.data().unwrap();
                    buffer.resize(min_size, 0);
                    continue;
                },
            };
        }
        root.reset_entry_readout().unwrap().unwrap();
        INIT = true;
        return root;
    } else {
        error("Failed to initialized FAT file system");
        INIT = false;
        return fs_init(bs);
    }
}
