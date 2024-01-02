# Get a list of all directories containing a Cargo.toml file
DIRS := $(shell find . -name 'Cargo.toml' -printf '%h\n')

# Redirect to null
TO_NULL = > /dev/null 2>&1

# Transfer to bytes
TO_BYTES = | tac -rs '..' | xxd -r -p

# Count the number of Rust projects
COUNT := $(words $(DIRS))

# Sub app target directory 
APP_TARGET=target/riscv64gc-unknown-none-elf/release

# Image dirtory
TARGET = ./target

# Default target
all: clean $(DIRS)
	@dd if=$(TARGET)/apps.tmp of=$(TARGET)/apps.bin conv=notrunc
	@mkdir -p ../arceos/payload
	@mv $(TARGET)/apps.bin ../arceos/payload/apps.bin
	@rm $(TARGET)/apps.tmp

# Rule to build a directory
.PHONY: $(DIRS)
target:
	@mkdir -p $(TARGET)
	@dd if=/dev/zero of=target/apps.bin bs=1M count=32 $(TO_NULL)
	@printf "%016x\n" $(COUNT) $(TO_BYTES) > $(TARGET)/apps.tmp

$(DIRS): target
	@cargo build --manifest-path $@/Cargo.toml \
		--target riscv64gc-unknown-none-elf \
		--release
	@rust-objcopy --binary-architecture=riscv64 \
		--strip-all -O binary $@/$(APP_TARGET)/$@ \
		$(TARGET)/$@.bin
	@APP_SIZE=$$(stat -c%s $(TARGET)/$@.bin) && \
	printf "%016x\n" $$APP_SIZE $(TO_BYTES) >> $(TARGET)/apps.tmp
	@dd if=$(TARGET)/$@.bin of=$(TARGET)/apps.tmp conv=notrunc oflag=append

# Clean target
clean:
	@for dir in $(DIRS); do \
		cargo clean --manifest-path $$dir/Cargo.toml; \
	done
	@rm -rf ./target

# Count target
count:
	@echo "Number of Rust projects in the 'app' directory: $(COUNT)"
