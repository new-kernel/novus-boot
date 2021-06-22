# Novus Boot

A bootloader intended for UEFI [Novusk](https://github.com/NathanMcMillan54/novusk)

``boot/`` - A bootloader for UEFI apps, kernels, OSes, etc...

``command/`` - For creating bootable UEFI images

``example/`` - Example UEFI OS

### Command:

Build:
```commandline
cd command/ && sh build.sh
```

Usage:
```commandline
novus_boot <path_to_efi_bootloader> <path_to_efi_kernel> <architecture_for_os>
```

### Bootloader:

Cargo.toml:
```toml
[dependencies]
novus-boot = "1.0.0"
```

src/main.rs:
```rust
#![no_std]
#![no_main]

pub extern crate novus_boot;

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {  }
}
```
