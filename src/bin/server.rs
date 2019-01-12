extern crate nix;
extern crate rustpkg;

use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::path::Path;
use std::os::unix::io::RawFd;

use nix::libc::off_t;
use nix::fcntl::{open, OFlag};
use nix::sys::epoll::*;
use nix::sys::stat::Mode;
use nix::sys::socket::{accept, bind, listen, recv, send, socket, shutdown};
use nix::sys::socket::SockFlag;
use nix::sys::socket::SockProtocol;
use nix::sys::socket::{AddressFamily, MsgFlags, SockAddr, SockType, Shutdown, InetAddr, IpAddr};
use nix::unistd::{lseek, read, Whence};

use rustpkg::AccessMap::AccessMap;
use rustpkg::SocketSet::*;

fn open_socket(path: &Path) -> RawFd {
	let listen_fd = socket(AddressFamily::Unix, SockType::Stream, SockFlag::empty(), None).unwrap();

    let addr_path = SockAddr::new_unix(path).unwrap();
    let b = bind(listen_fd, &addr_path).unwrap();
    let l = listen(listen_fd, 5);
    return listen_fd;
}


fn init_connection_thread(s: Arc<SocketSet>) -> JoinHandle<()> {
    let listen_epoll_fd = epoll_create().unwrap();
    let listen_fd = open_socket(Path::new("/tmp/test"));

    let mut event = EpollEvent::new(EpollFlags::EPOLLIN | EpollFlags::EPOLLET, 0);
    let _c = epoll_ctl(listen_epoll_fd, EpollOp::EpollCtlAdd, listen_fd, Some(&mut event));

    let handle = thread::spawn(move || {
		let mut event = [EpollEvent::empty()];
        loop {
    		let count = epoll_wait(listen_epoll_fd, &mut event, -1).unwrap();

    		for c in 0..count {
    			let id = event[c].data();
                println!("connect {}", id);
				let connected_fd = accept(listen_fd).unwrap();
				s.add(connected_fd);
    		}
        }
    });
    return handle;
}


/**
 * Must share the map between threads
 */
fn reply_thread(s: Arc<SocketSet>) -> JoinHandle<()> {
    let handle = thread::spawn(move || {

        loop {
    		s.wait();
        }

    });
    return handle;
}


fn start_loop() {
    let sockets = Arc::new(SocketSet::new());

    let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();

    thread_pool.push(init_connection_thread(sockets.clone()));
    thread_pool.push(reply_thread(sockets.clone()));

    for thread in thread_pool {
	     thread.join();
    }
}


fn main() {
    start_loop();
}
