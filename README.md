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
image.sh:
```shell
cargo build --my-target.json
mv target/my-target/debug/mykernel boot/mykernel.elf
cd boot && novus_boot mykernel.elf novus_boot.efi <arch-family>
```

Boot:
```commandline
boot/
|----.gitignore
|----novus_boot.efi
|----mykernel.elf
```