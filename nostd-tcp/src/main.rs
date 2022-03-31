#![no_std]
#![feature(lang_items)]
#![feature(start)]
#![feature(rustc_private)]
#![feature(default_alloc_error_handler)]
extern crate libc;
extern crate alloc;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use alloc::{boxed::Box};
use alloc::sync::Arc;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use core::option::Option;
use woke::{waker_ref, Woke};

use libc_print::std_name::*;

struct TestIO {
    waker: Option<Box<Waker>>
}

impl Future for TestIO {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.waker = Some(Box::new(cx.waker().clone()));
        Poll::Pending
    }
}



struct Test {
    val: usize
}

impl Woke for Test {
    fn wake_by_ref(_: &Arc<Self>) {
        println!("wake_by_ref()");
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

async fn test2() -> u32 {
   println!("test2");
   2
}

fn main() {
    println!("aa");

    let io = TestIO{waker: None};
    let fut = async {
        println!("async");
        let f = test2().await;
        io.await;
        fibonacci(100*f)
    };
    let mut futbox = Box::pin(fut);
    let a = Arc::new(Test{val: 1});
    loop {

        let waker = waker_ref(&a);
        let context = &mut Context::from_waker(&waker);
        let t = match futbox.as_mut().poll(context) {
            Poll::Ready(t) => t,
            Poll::Pending => 0,
        };
        println!("poll: {}", t);
    }
        
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
