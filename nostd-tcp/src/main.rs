#![no_std]
mod executor;

use {
    core::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    }
};
use libc_print::std_name::*;
use cstr_core::CString;



fn connect() -> libc::c_int {
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

    sock
}

struct Recv {}

impl Future for Recv {
    type Output = libc::c_int;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

fn recv(sock: libc::c_int) -> Recv {
    Recv{}
}

fn main() {
    executor::run(async {
        let sock = connect();
        recv(sock).await;
    });
}
