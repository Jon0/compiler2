use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::path::Path;
use std::os::unix::io::RawFd;

use AccessMap::*;


pub struct SocketSet {
    accessmap: RwLock<AccessMap>
}

impl SocketSet {

    pub fn new() -> Self {
        return SocketSet {
            accessmap: RwLock::new(AccessMap::new())
        }
    }

    pub fn wait(&self) {

		// a separate object prevents blocking all the time
		let handler = self.accessmap.read().unwrap().getWaiter();
		let id = handler.wait();

		self.accessmap.write().unwrap().trigger(id);
    }

    pub fn add(&self, fd: RawFd) {
		self.accessmap.write().unwrap().add(fd);
    }

}
