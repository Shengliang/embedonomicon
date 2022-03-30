#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;

use rt::entry;
entry!(main);

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

fn main() -> ! {
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };

    intrinsics::abort()
}

#[no_mangle]
pub extern "C" fn HardFaultHandler() -> ! {
    // do something interesting here
    let _p = RODATA;
    loop {}
}
