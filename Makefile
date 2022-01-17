TARGET =?

all:
	@ cargo build -p gen_image --target x86_64-unknown-linux-gnu
	@ echo "Compiling Novus Boot for $(TARGET)"
	@ cargo build --target $(TARGET) -Z build-std=core \
                                         -Z build-std-features=compiler-builtins-mem
	@ sudo mv target/x86_64-unknown-linux-gnu/debug/gen_image /bin/novus_boot_gen_image

clean:
	@ cargo clean
