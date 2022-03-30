
all:
	cargo +nightly build
	#cargo +nightly objdump --bin app -- -d --no-show-raw-insn
	cargo +nightly objdump --bin app --release -- -d --no-show-raw-insn --print-imm-hex
	# cargo +nightly objdump --bin app -- -s --section .vector_table
	cargo +nightly objdump --bin app --release -- -s -j .vector_table
	cargo objdump --bin app -- -t | grep '\.log'


tool:
	cargo objdump -- --version
	rustc --print target-list | column
	rustup target list | column

tool_nightly:
	cargo +nightly objdump -- --version
	rustc +nightly --print target-list | column
	rustc +nightly -Z unstable-options --print target-spec-json --target thumbv7m-none-eabi
