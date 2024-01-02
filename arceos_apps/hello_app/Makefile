.PHONY: build

HELLO_APP_SIZE=$$(stat -c%s hello_app.bin)
APP_COUNT=0

build:
	cargo build --target riscv64gc-unknown-none-elf --release
	rust-objcopy --binary-architecture=riscv64 --strip-all -O binary target/riscv64gc-unknown-none-elf/release/hello_app ./hello_app.bin
	# Get the size of hello_app.bin
	# Write the size to the front of apps.tmp
	printf "%016x\n" $(HELLO_APP_SIZE) | tac -rs '..' | xxd -r -p > apps.tmp
	dd if=./hello_app.bin of=./apps.tmp conv=notrunc oflag=append
	dd if=/dev/zero of=./apps.bin bs=1M count=32
	dd if=./apps.tmp of=./apps.bin conv=notrunc
	mkdir -p ../arceos/payload
	mv ./apps.bin ../arceos/payload/apps.bin
	rm ./apps.tmp

