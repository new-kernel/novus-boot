use uefi::prelude::BootServices;
use uefi::proto::media::file::Directory;
use uefi::proto::media::fs::SimpleFileSystem;

pub fn file_system_init(boot_services: &BootServices) -> &mut SimpleFileSystem {
    if let Ok(sfs) = boot_services.locate_protocol::<SimpleFileSystem>() {
        let sfs = sfs.expect("Cannot get Simple File System");
        let sfs = unsafe { &mut *sfs.get() };

        return sfs;
    } else { panic!("Couldn't get root directory"); }
}

pub fn root_dir(bs: &BootServices) -> Directory {
    return file_system_init(bs).open_volume().unwrap().unwrap();
}

