all: build
	@echo
	@echo "Test with this line:"
	@echo
	@echo "   LD_PRELOAD=target/release/librs_preload_example.so hostname"
	@echo

build:
	cargo build --release

clean:
	cargo clean
