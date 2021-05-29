#!/bin/bash

cargo build
sudo mv target/debug/novus_boot /bin/novus_boot
