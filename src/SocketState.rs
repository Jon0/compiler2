use std::sync::Arc;
use std::sync::RwLock;
use std::os::unix::io::RawFd;

use nix::unistd::{read, write};

use Action::*;
use MessageStream::*;

////
/// connections are each mapped to a socket state
/// these must be mutable for performing read write operations
///
#[derive(Debug, Clone)]
pub struct SocketState {
	fd: RawFd,
	stream: Arc<MessageStream>
}

impl SocketState {

	pub fn new(fd: RawFd) -> Self {
		return SocketState {
			fd: fd,
			stream: Arc::new(MessageStream::new())
		}
	}

	pub fn try_read(&mut self) {
		let mut buf: [u8; 256] = [0; 256];
		let result = read(self.fd, &mut buf).unwrap();

		// todo try read again if buffer is completely used

		Arc::make_mut(&mut self.stream).recv(&buf[0..result]);
	}


	pub fn read_requests(&mut self) -> Vec<Box<Request>> {
		self.try_read();

		return vec![Box::new(ListRequest::new())];
	}


	pub fn write(&self, s: String) {
		let _r = write(self.fd, s.as_bytes());
	}

}
