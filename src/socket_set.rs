use std::sync::RwLock;
use std::os::unix::io::RawFd;

use access_map::*;
use socket_state::*;

pub struct SocketSet {
	accessmap: RwLock<AccessMap>
}

impl SocketSet {

	pub fn new() -> Self {
		return SocketSet {
			accessmap: RwLock::new(AccessMap::new())
		}
	}

	pub fn wait(&self) -> SocketState {

		// a separate object prevents blocking all the time
		let waiter = self.accessmap.read().unwrap().get_waiter();
		let id = waiter.wait();

		return self.accessmap.write().unwrap().get_socket(id);
	}

	pub fn add(&self, fd: RawFd) {
		self.accessmap.write().unwrap().add(fd);
	}

}
