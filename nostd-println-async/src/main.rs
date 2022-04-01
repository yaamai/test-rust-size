#![no_std]
#![feature(lang_items)]
#![feature(start)]
#![feature(rustc_private)]
#![feature(default_alloc_error_handler)]
extern crate libc;

use libc_print::std_name::*;
use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

async fn func1() -> u32 {
    123
}

async fn main() {
    println!("test {}", func1().await);
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    executor::run(main());
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
