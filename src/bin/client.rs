extern crate nix;
extern crate rustpkg;

use std::path::Path;
use std::os::unix::io::RawFd;
use nix::sys::socket::{connect, socket};
use nix::sys::socket::SockFlag;
use nix::sys::socket::{AddressFamily, SockAddr, SockType};
use nix::unistd::{read, write};

fn connect_socket(path: &Path) -> RawFd {
	let fd = socket(AddressFamily::Unix, SockType::Stream, SockFlag::empty(), None).unwrap();

	let addr_path = SockAddr::new_unix(path).unwrap();
	connect(fd, &addr_path).expect("failed to connect");
	return fd;
}


fn main() {
	let mut recv = [255; 0];
	let msg = "test".as_bytes();

	let fd = connect_socket(Path::new("/tmp/test"));

	let _b = write(fd, msg).expect("failed to write socket");


	let len = read(fd, &mut recv).expect("failed to read socket");

	println!("reply: {:?}", &recv[0..len]);
}