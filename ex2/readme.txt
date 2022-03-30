
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

 cargo +nightly build
 cargo +nightly objdump --bin app -- -d --no-show-raw-insn
 cargo +nightly objdump --bin app -- -s --section .vector_table

cargo +nightly objdump --bin app -- -d --no-show-raw-insn
app:	file format elf32-littlearm

Disassembly of section .text:

0000003c <app::main::h1249caf810d998a6>:
      3c:      	sub	sp, #16
      3e:      	movw	r0, #1024
      42:      	movt	r0, #0
      46:      	ldr	r1, [r0]
      48:      	ldr	r0, [r0, #4]
      4a:      	str	r1, [sp]
      4c:      	str	r0, [sp, #4]
      4e:      	movw	r0, #0
      52:      	movt	r0, #8192
      56:      	str	r0, [sp, #8]
      58:      	movw	r0, #2
      5c:      	movt	r0, #8192
      60:      	str	r0, [sp, #12]
      62:      	trap
      64:      	trap

00000066 <HardFault>:
      66:      	sub	sp, #8
      68:      	movw	r0, #1024
      6c:      	movt	r0, #0
      70:      	ldr	r1, [r0]
      72:      	ldr	r0, [r0, #4]
      74:      	str	r1, [sp]
      76:      	str	r0, [sp, #4]
      78:      	b	0x7a <HardFault+0x14>   @ imm = #-2
      7a:      	b	0x7a <HardFault+0x14>   @ imm = #-4

0000007c <main>:
      7c:      	push	{r7, lr}
      7e:      	mov	r7, sp
      80:      	sub	sp, #8
      82:      	movw	r0, #61
      86:      	movt	r0, #0
      8a:      	str	r0, [sp, #4]
      8c:      	bl	0x3c <app::main::h1249caf810d998a6> @ imm = #-84
      90:      	trap

00000092 <Reset>:
      92:      	push	{r7, lr}
      94:      	mov	r7, sp
      96:      	sub	sp, #16
      98:      	movw	r1, #0
      9c:      	movt	r1, #8192
      a0:      	movw	r0, #1
      a4:      	movt	r0, #8192
      a8:      	subs	r2, r0, r1
      aa:      	str	r2, [sp, #4]
      ac:      	cmp	r0, r1
      ae:      	blo	0xc6 <Reset+0x34>       @ imm = #20
      b0:      	b	0xb2 <Reset+0x20>       @ imm = #-2
      b2:      	ldr	r2, [sp, #4]
      b4:      	str	r2, [sp, #8]
      b6:      	movw	r0, #0
      ba:      	movt	r0, #8192
      be:      	movs	r1, #0
      c0:      	bl	0x13c <core::intrinsics::write_bytes::h90587b5306b94a58> @ imm = #120
      c4:      	b	0xde <Reset+0x4c>       @ imm = #22
      c6:      	movw	r0, #1088
      ca:      	movt	r0, #0
      ce:      	movw	r2, #1072
      d2:      	movt	r2, #0
      d6:      	movs	r1, #33
      d8:      	bl	0x18a <core::panicking::panic::h4dd733aff9ad7c1d> @ imm = #174
      dc:      	trap
      de:      	movw	r1, #2
      e2:      	movt	r1, #8192
      e6:      	movw	r0, #4
      ea:      	movt	r0, #8192
      ee:      	subs	r2, r0, r1
      f0:      	str	r2, [sp]
      f2:      	cmp	r0, r1
      f4:      	blo	0x112 <Reset+0x80>      @ imm = #26
      f6:      	b	0xf8 <Reset+0x66>       @ imm = #-2
      f8:      	ldr	r2, [sp]
      fa:      	str	r2, [sp, #12]
      fc:      	movw	r0, #1156
     100:      	movt	r0, #0
     104:      	movw	r1, #2
     108:      	movt	r1, #8192
     10c:      	bl	0x15c <core::intrinsics::copy_nonoverlapping::h07b589f40a9f0150> @ imm = #76
     110:      	b	0x12a <Reset+0x98>      @ imm = #22
     112:      	movw	r0, #1088
     116:      	movt	r0, #0
     11a:      	movw	r2, #1124
     11e:      	movt	r2, #0
     122:      	movs	r1, #33
     124:      	bl	0x18a <core::panicking::panic::h4dd733aff9ad7c1d> @ imm = #98
     128:      	trap
     12a:      	bl	0x7c <main>             @ imm = #-178
     12e:      	trap

00000130 <rust_begin_unwind>:
     130:      	sub	sp, #4
     132:      	str	r0, [sp]
     134:      	b	0x136 <rust_begin_unwind+0x6> @ imm = #-2
     136:      	b	0x136 <rust_begin_unwind+0x6> @ imm = #-4

00000138 <UsageFault>:
     138:      	b	0x13a <UsageFault+0x2>  @ imm = #-2
     13a:      	b	0x13a <UsageFault+0x2>  @ imm = #-4

0000013c <core::intrinsics::write_bytes::h90587b5306b94a58>:
     13c:      	push	{r7, lr}
     13e:      	mov	r7, sp
     140:      	sub	sp, #16
     142:      	str	r2, [sp]
     144:      	mov	r2, r1
     146:      	ldr	r1, [sp]
     148:      	str	r0, [sp, #4]
     14a:      	strb	r2, [r7, #-5]
     14e:      	str	r1, [sp, #12]
     150:      	uxtb	r2, r2
     152:      	bl	0x1e6 <__aeabi_memset>  @ imm = #144
     156:      	b	0x158 <core::intrinsics::write_bytes::h90587b5306b94a58+0x1c> @ imm = #-2
     158:      	add	sp, #16
     15a:      	pop	{r7, pc}

0000015c <core::intrinsics::copy_nonoverlapping::h07b589f40a9f0150>:
     15c:      	push	{r7, lr}
     15e:      	mov	r7, sp
     160:      	sub	sp, #16
     162:      	str	r1, [sp]
     164:      	mov	r1, r0
     166:      	ldr	r0, [sp]
     168:      	str	r1, [sp, #4]
     16a:      	str	r0, [sp, #8]
     16c:      	str	r2, [sp, #12]
     16e:      	bl	0x3de <__aeabi_memcpy>  @ imm = #620
     172:      	add	sp, #16
     174:      	pop	{r7, pc}

00000176 <core::ptr::drop_in_place<&core::iter::adapters::copied::Copied<core::slice::iter::Iter<u8>>>::h0207d9dfe5eaa9d0>:
     176:      	bx	lr

00000178 <<T as core::any::Any>::type_id::hf321cd0861761fcf>:
     178:      	movw	r0, #38821
     17c:      	movw	r1, #55927
     180:      	movt	r0, #11197
     184:      	movt	r1, #17858
     188:      	bx	lr

0000018a <core::panicking::panic::h4dd733aff9ad7c1d>:
     18a:      	push	{r7, lr}
     18c:      	mov	r7, sp
     18e:      	sub	sp, #32
     190:      	mov	r12, r2
     192:      	movw	r2, #1140
     196:      	movt	r2, #0
     19a:      	movs	r3, #0
     19c:      	strd	r0, r1, [sp, #24]
     1a0:      	mov	r0, sp
     1a2:      	str	r2, [sp, #16]
     1a4:      	movs	r2, #1
     1a6:      	mov	r1, r12
     1a8:      	str	r2, [sp, #4]
     1aa:      	add	r2, sp, #24
     1ac:      	str	r3, [sp, #20]
     1ae:      	strd	r3, r3, [sp, #8]
     1b2:      	str	r2, [sp]
     1b4:      	bl	0x1ba <core::panicking::panic_fmt::h81242f59dd71f87b> @ imm = #2
     1b8:      	trap

000001ba <core::panicking::panic_fmt::h81242f59dd71f87b>:
     1ba:      	push	{r7, lr}
     1bc:      	mov	r7, sp
     1be:      	sub	sp, #24
     1c0:      	strd	r0, r1, [sp, #12]
     1c4:      	movw	r0, #1140
     1c8:      	movt	r0, #0
     1cc:      	movs	r2, #1
     1ce:      	str	r0, [sp, #8]
     1d0:      	movw	r0, #1140
     1d4:      	movt	r0, #0
     1d8:      	strb.w	r2, [sp, #20]
     1dc:      	str	r0, [sp, #4]
     1de:      	add	r0, sp, #4
     1e0:      	bl	0x130 <rust_begin_unwind> @ imm = #-180
     1e4:      	trap

000001e6 <__aeabi_memset>:
     1e6:      	b.w	0x1ee <compiler_builtins::arm::__aeabi_memset::h9a4c063606636b41> @ imm = #4

000001ea <compiler_builtins::arm::__aeabi_memcpy::ha2001aafadab4da8>:
     1ea:      	b.w	0x1f8 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6> @ imm = #10

000001ee <compiler_builtins::arm::__aeabi_memset::h9a4c063606636b41>:
     1ee:      	mov	r3, r1
     1f0:      	mov	r1, r2
     1f2:      	mov	r2, r3
     1f4:      	b.w	0x340 <compiler_builtins::mem::memset::haed27158e012070e> @ imm = #328

000001f8 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6>:
     1f8:      	push	{r4, r5, r6, r7, lr}
     1fa:      	add	r7, sp, #12
     1fc:      	push.w	{r8, r9, r10}
     200:      	cmp	r2, #15
     202:      	bls	0x2cc <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0xd4> @ imm = #198
     204:      	rsbs	r3, r0, #0
     206:      	ands	r4, r3, #3
     20a:      	add.w	r12, r0, r4
     20e:      	beq	0x23e <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x46> @ imm = #44
     210:      	mov	r3, r0
     212:      	mov	r6, r1
     214:      	ldrb	r5, [r6]
     216:      	strb	r5, [r3], #1
     21a:      	cmp	r3, r12
     21c:      	bhs	0x23e <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x46> @ imm = #30
     21e:      	ldrb	r5, [r6, #1]
     220:      	strb	r5, [r3], #1
     224:      	cmp	r3, r12
     226:      	ittt	lo
     228:      	ldrblo	r5, [r6, #2]
     22a:      	strblo	r5, [r3], #1
     22e:      	cmplo	r3, r12
     230:      	bhs	0x23e <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x46> @ imm = #10
     232:      	ldrb	r5, [r6, #3]
     234:      	adds	r6, #4
     236:      	strb	r5, [r3], #1
     23a:      	cmp	r3, r12
     23c:      	blo	0x214 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x1c> @ imm = #-44
     23e:      	sub.w	lr, r2, r4
     242:      	add.w	r9, r1, r4
     246:      	bic	r8, lr, #3
     24a:      	add.w	r3, r12, r8
     24e:      	lsls.w	r2, r9, #30
     252:      	beq	0x2d2 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0xda> @ imm = #124
     254:      	cmp.w	r8, #1
     258:      	blt	0x304 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x10c> @ imm = #168
     25a:      	movs	r2, #24
     25c:      	and.w	r10, r2, r9, lsl #3
     260:      	bic	r2, r9, #3
     264:      	lsl.w	r6, r9, #3
     268:      	rsbs	r6, r6, #0
     26a:      	add.w	r5, r2, #8
     26e:      	ldr	r2, [r2]
     270:      	and	r6, r6, #24
     274:      	lsr.w	r1, r2, r10
     278:      	ldr	r2, [r5, #-4]
     27c:      	lsl.w	r4, r2, r6
     280:      	orrs	r1, r4
     282:      	str	r1, [r12], #4
     286:      	cmp	r12, r3
     288:      	bhs	0x304 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x10c> @ imm = #120
     28a:      	lsr.w	r1, r2, r10
     28e:      	ldr	r2, [r5]
     290:      	lsl.w	r4, r2, r6
     294:      	orrs	r1, r4
     296:      	str	r1, [r12], #4
     29a:      	cmp	r12, r3
     29c:      	itttt	lo
     29e:      	lsrlo.w	r1, r2, r10
     2a2:      	ldrlo	r2, [r5, #4]
     2a4:      	lsllo.w	r4, r2, r6
     2a8:      	orrlo	r1, r4
     2aa:      	itt	lo
     2ac:      	strlo	r1, [r12], #4
     2b0:      	cmplo	r12, r3
     2b2:      	bhs	0x304 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x10c> @ imm = #78
     2b4:      	lsr.w	r1, r2, r10
     2b8:      	ldr	r2, [r5, #8]
     2ba:      	adds	r5, #16
     2bc:      	lsl.w	r4, r2, r6
     2c0:      	orrs	r1, r4
     2c2:      	str	r1, [r12], #4
     2c6:      	cmp	r12, r3
     2c8:      	blo	0x274 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x7c> @ imm = #-88
     2ca:      	b	0x304 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x10c> @ imm = #54
     2cc:      	mov	r3, r0
     2ce:      	cbnz	r2, 0x30e <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x116> @ imm = #60
     2d0:      	b	0x33a <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x142> @ imm = #102
     2d2:      	cmp.w	r8, #1
     2d6:      	blt	0x304 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x10c> @ imm = #42
     2d8:      	mov	r4, r9
     2da:      	ldr	r1, [r4]
     2dc:      	str	r1, [r12], #4
     2e0:      	cmp	r12, r3
     2e2:      	bhs	0x304 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x10c> @ imm = #30
     2e4:      	ldr	r1, [r4, #4]
     2e6:      	str	r1, [r12], #4
     2ea:      	cmp	r12, r3
     2ec:      	ittt	lo
     2ee:      	ldrlo	r1, [r4, #8]
     2f0:      	strlo	r1, [r12], #4
     2f4:      	cmplo	r12, r3
     2f6:      	bhs	0x304 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x10c> @ imm = #10
     2f8:      	ldr	r1, [r4, #12]
     2fa:      	adds	r4, #16
     2fc:      	str	r1, [r12], #4
     300:      	cmp	r12, r3
     302:      	blo	0x2da <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0xe2> @ imm = #-44
     304:      	add.w	r1, r9, r8
     308:      	and	r2, lr, #3
     30c:      	cbz	r2, 0x33a <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x142> @ imm = #42
     30e:      	add	r2, r3
     310:      	ldrb	r6, [r1]
     312:      	strb	r6, [r3], #1
     316:      	cmp	r3, r2
     318:      	bhs	0x33a <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x142> @ imm = #30
     31a:      	ldrb	r6, [r1, #1]
     31c:      	strb	r6, [r3], #1
     320:      	cmp	r3, r2
     322:      	ittt	lo
     324:      	ldrblo	r6, [r1, #2]
     326:      	strblo	r6, [r3], #1
     32a:      	cmplo	r3, r2
     32c:      	bhs	0x33a <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x142> @ imm = #10
     32e:      	ldrb	r6, [r1, #3]
     330:      	adds	r1, #4
     332:      	strb	r6, [r3], #1
     336:      	cmp	r3, r2
     338:      	blo	0x310 <compiler_builtins::mem::memcpy::h1afe9a166fca87c6+0x118> @ imm = #-44
     33a:      	pop.w	{r8, r9, r10}
     33e:      	pop	{r4, r5, r6, r7, pc}

00000340 <compiler_builtins::mem::memset::haed27158e012070e>:
     340:      	push	{r4, r6, r7, lr}
     342:      	add	r7, sp, #8
     344:      	cmp	r2, #15
     346:      	bls	0x3b6 <compiler_builtins::mem::memset::haed27158e012070e+0x76> @ imm = #108
     348:      	rsbs	r3, r0, #0
     34a:      	ands	lr, r3, #3
     34e:      	add.w	r12, r0, lr
     352:      	beq	0x376 <compiler_builtins::mem::memset::haed27158e012070e+0x36> @ imm = #32
     354:      	mov	r3, r0
     356:      	strb	r1, [r3], #1
     35a:      	cmp	r3, r12
     35c:      	bhs	0x376 <compiler_builtins::mem::memset::haed27158e012070e+0x36> @ imm = #22
     35e:      	strb	r1, [r3], #1
     362:      	cmp	r3, r12
     364:      	itt	lo
     366:      	strblo	r1, [r3], #1
     36a:      	cmplo	r3, r12
     36c:      	bhs	0x376 <compiler_builtins::mem::memset::haed27158e012070e+0x36> @ imm = #6
     36e:      	strb	r1, [r3], #1
     372:      	cmp	r3, r12
     374:      	blo	0x356 <compiler_builtins::mem::memset::haed27158e012070e+0x16> @ imm = #-34
     376:      	sub.w	lr, r2, lr
     37a:      	bic	r2, lr, #3
     37e:      	add.w	r3, r12, r2
     382:      	cmp	r2, #1
     384:      	blt	0x3ae <compiler_builtins::mem::memset::haed27158e012070e+0x6e> @ imm = #38
     386:      	uxtb	r2, r1
     388:      	mov.w	r4, #16843009
     38c:      	muls	r2, r4, r2
     38e:      	str	r2, [r12], #4
     392:      	cmp	r12, r3
     394:      	bhs	0x3ae <compiler_builtins::mem::memset::haed27158e012070e+0x6e> @ imm = #22
     396:      	str	r2, [r12], #4
     39a:      	cmp	r12, r3
     39c:      	itt	lo
     39e:      	strlo	r2, [r12], #4
     3a2:      	cmplo	r12, r3
     3a4:      	bhs	0x3ae <compiler_builtins::mem::memset::haed27158e012070e+0x6e> @ imm = #6
     3a6:      	str	r2, [r12], #4
     3aa:      	cmp	r12, r3
     3ac:      	blo	0x38e <compiler_builtins::mem::memset::haed27158e012070e+0x4e> @ imm = #-34
     3ae:      	and	r2, lr, #3
     3b2:      	cbnz	r2, 0x3ba <compiler_builtins::mem::memset::haed27158e012070e+0x7a> @ imm = #4
     3b4:      	b	0x3dc <compiler_builtins::mem::memset::haed27158e012070e+0x9c> @ imm = #36
     3b6:      	mov	r3, r0
     3b8:      	cbz	r2, 0x3dc <compiler_builtins::mem::memset::haed27158e012070e+0x9c> @ imm = #32
     3ba:      	add	r2, r3
     3bc:      	strb	r1, [r3], #1
     3c0:      	cmp	r3, r2
     3c2:      	bhs	0x3dc <compiler_builtins::mem::memset::haed27158e012070e+0x9c> @ imm = #22
     3c4:      	strb	r1, [r3], #1
     3c8:      	cmp	r3, r2
     3ca:      	itt	lo
     3cc:      	strblo	r1, [r3], #1
     3d0:      	cmplo	r3, r2
     3d2:      	bhs	0x3dc <compiler_builtins::mem::memset::haed27158e012070e+0x9c> @ imm = #6
     3d4:      	strb	r1, [r3], #1
     3d8:      	cmp	r3, r2
     3da:      	blo	0x3bc <compiler_builtins::mem::memset::haed27158e012070e+0x7c> @ imm = #-34
     3dc:      	pop	{r4, r6, r7, pc}

000003de <__aeabi_memcpy>:
     3de:      	b.w	0x1ea <compiler_builtins::arm::__aeabi_memcpy::ha2001aafadab4da8> @ imm = #-504


 cargo +nightly objdump --bin app -- -s --section .vector_table

app:	file format elf32-littlearm
Contents of section .vector_table:
 0000 00000120 39010000 67000000 39010000  ... 9...g...9...
 0010 39010000 39010000 00000000 00000000  9...9...........
 0020 00000000 00000000 39010000 00000000  ........9.......
 0030 00000000 39010000 39010000           ....9...9...
