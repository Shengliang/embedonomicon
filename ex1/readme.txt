
://docs.rust-embedded.org/embedonomicon/preface.html

# Rust toolchain
# If you start from scratch, get rustup from https://rustup.rs/
$ rustup +nightly default stable

# toolchain should be newer than this one
ssl@ssl:/scratch/ssl/embedonomicon/ex2$ rustc -V
rustc 1.59.0 (9d1b2106e 2022-02-23)
ssl@ssl:/scratch/ssl/embedonomicon/ex2$ rustc +nightly -V
rustc 1.61.0-nightly (9c06e1ba4 2022-03-29)

$ rustup +nightly target add thumbv7m-none-eabi

# cargo-binutils
$ cargo +nightly install cargo-binutils

$ rustup +nightly component add llvm-tools-preview

ssl@ssl:/scratch/ssl/embedonomicon/ex1$ cargo +nightly build
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
ssl@ssl:/scratch/ssl/embedonomicon/ex1$ cargo +nightly objdump --bin app -- -d --no-show-raw-insn
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s

app:    file format elf32-littlearm

Disassembly of section .text:

00000008 <Reset>:
       8:       sub     sp, #4
       a:       movs    r0, #42
       c:       str     r0, [sp]
       e:       b       0x10 <Reset+0x8>        @ imm = #-2
      10:       b       0x10 <Reset+0x8>        @ imm = #-4
ssl@ssl:/scratch/ssl/embedonomicon/ex1$ cargo +nightly objdump --bin app -- -s --section .vector_table
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s

app:    file format elf32-littlearm
Contents of section .vector_table:
 0000 00000120 09000000                    ... ....

