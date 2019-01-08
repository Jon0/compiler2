use std::collections::HashMap;
use std::os::unix::io::RawFd;

use nix::sys::epoll::*;

pub struct AccessHandler {
    epoll_fd: RawFd
}

impl AccessHandler {

    pub fn new(fd: RawFd) -> Self {
        return AccessHandler {
            epoll_fd: fd
        }
    }

    pub fn wait(&self) -> u64 {
        let mut event = [EpollEvent::empty()];

        let count = epoll_wait(self.epoll_fd, &mut event, -1).unwrap();
        for c in 0..count {
            return event[c].data();
        }

        return 0;
    }

}

pub struct AccessMap {
    epoll_fd: RawFd,
    map: HashMap<u64, String>,
    next_id: u64,
}


impl AccessMap {

    pub fn new() -> Self {
        return AccessMap {
            epoll_fd: epoll_create().unwrap(),
            map: HashMap::new(),
            next_id: 1
        }
    }

    pub fn add(&mut self, fd: RawFd) {
        let id = self.next_id;
        self.next_id += 1;
        self.map.insert(id, "todo".to_string());

        println!("add {}", fd);
        let mut event = EpollEvent::new(EpollFlags::EPOLLIN | EpollFlags::EPOLLET, id);
        let c = epoll_ctl(self.epoll_fd, EpollOp::EpollCtlAdd, fd, Some(&mut event));
    }


    pub fn getHandler(&self) -> AccessHandler {
        return AccessHandler::new(self.epoll_fd);
    }


    pub fn trigger(&mut self, id: u64) {
        println!("recv {}", id);
        println!("mapped to {}", self.map.get(&id).unwrap())
    }
}
