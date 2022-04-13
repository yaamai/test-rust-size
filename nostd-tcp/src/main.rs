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

use alloc::{boxed::Box};
use alloc::sync::Arc;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use core::option::Option;
use woke::{waker_ref, Woke};
use core::ptr;

use libc_print::std_name::*;
use cstr_core::CString;


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
    let sock = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let mut hints: libc::addrinfo = unsafe { core::mem::zeroed() };
    hints.ai_socktype = libc::SOCK_STREAM;
    let mut result = core::ptr::null_mut();
    let host = CString::new("example.com").unwrap();
    let port = CString::new("80").unwrap();
    let rc = unsafe { libc::getaddrinfo(host.as_ptr(), port.as_ptr(), &hints, &mut result) };
    println!("{} {}", rc, unsafe { (result as *mut libc::addrinfo).as_ref().unwrap().ai_addrlen as usize});

    // let addrstor: &libc::sockaddr_storage = unsafe { core::mem::transmute((result as *mut libc::addrinfo).as_ref().unwrap().ai_addr) };
    let addrinfo = unsafe { (result as *mut libc::addrinfo).as_ref().unwrap() };
    // let addrinfo: *const libc::sockaddr = *(addrstor as *const _ as *const libc::sockaddr);
    let rc2 = unsafe { libc::connect(sock, addrinfo.ai_addr, addrinfo.ai_addrlen) };
    println!("{}", rc2);
    // libc::connect(sock);
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
    println!("crashed");
    loop {}
}
