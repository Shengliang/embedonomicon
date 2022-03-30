
all:
	arm-none-eabi-as -march=armv7-m asm.s -o asm.o
	arm-none-eabi-ar crs librt.a asm.o
	arm-none-eabi-objdump -Cd librt.a
	cargo +nightly build
