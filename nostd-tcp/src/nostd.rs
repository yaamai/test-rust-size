#![no_std]
#![feature(lang_items)]
#![feature(start)]
#![feature(rustc_private)]
#![feature(default_alloc_error_handler)]
extern crate libc;
extern crate alloc;

use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("crashed");
    loop {}
}
