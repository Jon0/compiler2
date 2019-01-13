use std::os::unix::io::RawFd;

use nix::unistd::{read, write};

use Action::*;


pub struct Parser {

}


#[derive(Debug, Copy, Clone)]
pub struct SocketState {
	fd: RawFd
	// parser: RwLock<Parser>
}

impl SocketState {

	pub fn new(fd: RawFd) -> Self {
		return SocketState {
			fd: fd
		}
	}


	pub fn try_read(&self) -> String {
		let mut buf: [u8; 256] = [0; 256];
		let result = read(self.fd, &mut buf).unwrap();

		return String::from_utf8(buf[0..result].to_vec()).unwrap();
	}


	pub fn read_requests(&self) -> Vec<Box<Request>> {
		return vec![Box::new(ListRequest::new())];
	}


	pub fn write(&self, s: String) {
		let _r = write(self.fd, s.as_bytes());
	}

}
