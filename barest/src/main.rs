#![no_std]
#![no_main]
//#![feature(core_intrinsics)] //feature here because is outside standard lib

//use core::intrinsics;
use x86_64::instructions::hlt;

static HELLO: &[u8] = b"Hello World! This is Abdullah's first Kernel w!ithout host OS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;

    // unsafe {
    //     framebuffer.offset(1).write_volatile(0x30);
    // }

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *framebuffer.offset(i as isize * 2) = byte;
            *framebuffer.offset(i as isize * 2 + 1) = 0xb; //write with light cyan colour. Cn be changed
        }
    }
    loop {
        hlt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}