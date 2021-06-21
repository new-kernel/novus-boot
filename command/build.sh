#!/bin/bash

cargo build
sudo mv target/debug/novus_boot_command /bin/novus_boot
