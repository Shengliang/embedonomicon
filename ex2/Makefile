
all:
	cargo +nightly build
	#cargo +nightly objdump --bin app -- -d --no-show-raw-insn
	cargo +nightly objdump --bin app --release -- -d --no-show-raw-insn --print-imm-hex
	# cargo +nightly objdump --bin app -- -s --section .vector_table
	cargo +nightly objdump --bin app --release -- -s -j .vector_table
	cargo objdump --bin app -- -t | grep '\.log'
