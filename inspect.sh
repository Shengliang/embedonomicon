cargo +nightly objdump --bin app -- -d --no-show-raw-insn
cargo +nightly objdump --bin app -- -s --section .vector_table

exit
qemu-system-arm \
      -cpu cortex-m3 \
      -machine lm3s6965evb \
      -gdb tcp::3333 \
      -S \
      -nographic \
      -kernel target/thumbv7m-none-eabi/debug/app
