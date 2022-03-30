
all:
	cargo +nightly build
	arm-none-eabi-as -march=armv7-m asm.s -o asm.o
	arm-none-eabi-ar crs librta.a asm.o
	arm-none-eabi-objdump -Cd librt.a
