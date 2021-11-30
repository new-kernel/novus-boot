use std::{fs, io, path::Path};

fn create_fat_filesystem(fat_path: &Path, efi_file: &Path, arch: String) {
    // retrieve size of `.efi` file and round it up
    let efi_size = fs::metadata(&efi_file).unwrap().len();
    let mb = 1024 * 1024; // size of a megabyte
    // round it to next megabyte
    let efi_size_rounded = ((efi_size - 1) / mb + 1) * mb;

    // create new filesystem image file at the given path and set its length
    let fat_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&fat_path)
        .unwrap();
    fat_file.set_len(efi_size_rounded).unwrap();

    // create new FAT file system and open it
    let format_options = fatfs::FormatVolumeOptions::new();
    fatfs::format_volume(&fat_file, format_options).unwrap();
    let filesystem = fatfs::FileSystem::new(&fat_file, fatfs::FsOptions::new()).unwrap();

    // copy EFI file to FAT filesystem
    let root_dir = filesystem.root_dir();
    root_dir.create_dir("efi").unwrap();
    root_dir.create_dir("efi/boot").unwrap();
    if arch == String::from("x86") {
        let mut bootx64 = root_dir.create_file("efi/boot/bootx64.efi").unwrap();
        bootx64.truncate().unwrap();
        io::copy(&mut fs::File::open(&efi_file).unwrap(), &mut bootx64).unwrap();
    } else if arch == String::from("aarch64") {
        let mut bootaa64 = root_dir.create_file("efi/boot/bootaa64.efi").unwrap();
        bootaa64.truncate().unwrap();
        io::copy(&mut fs::File::open(&efi_file).unwrap(), &mut bootaa64).unwrap();
    } else {
        println!("{} - Not a supported architecture, use 'aarch64' or 'x86'", arch);
        exit(0);
    }
}

use std::{convert::TryFrom, fs::File, io::Seek};

use std::path::PathBuf;
use std::fmt::Debug;
use std::process::exit;

fn main() {
    println!("Creating bootable UEFI image...");
    // take efi file path as command line argument
    let mut args = std::env::args();
    let _exe_name = args.next().unwrap();
    let efi_path = PathBuf::from(args.next()
        .expect("path to `.efi` files must be given as argument"));

    let arch = args.next().unwrap();

    println!("Creating image from {:?}...", efi_path);

    let fat_path = efi_path.with_extension("fat");
    let disk_path = fat_path.with_extension("img");

    println!("Creating FAT file system...");
    create_fat_filesystem(&fat_path, &efi_path, arch);

    println!("Created images:");
    println!("    FAT image: {:?}", efi_path.with_extension("fat"));
    println!("    Image: {:?}", efi_path.with_extension("img"));
}
