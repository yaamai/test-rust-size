#![no_std]
#![feature(lang_items)]
#![feature(start)]
#![feature(rustc_private)]
extern crate libc;

use libc_print::std_name::*;

fn main() {
    println!("aa");
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
