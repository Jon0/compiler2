use std::collections::HashMap;
use std::os::unix::io::RawFd;

use nix::sys::epoll::*;

pub struct AccessMap {
    epoll_fd: RawFd,
    map: HashMap<u64, String>,
    next_id: u64,
}


impl AccessMap {

    pub fn new() -> Self {
        return AccessMap {
            epoll_fd: 0,
            map: HashMap::new(),
            next_id: 1
        }
    }

    pub fn add(&mut self, fd: RawFd) {
        let id = self.next_id;
        self.next_id += 1;
        self.map.insert(id, "todo".to_string());

        let mut event = EpollEvent::new(EpollFlags::EPOLLIN | EpollFlags::EPOLLET, id);
        let c = epoll_ctl(self.epoll_fd, EpollOp::EpollCtlAdd, fd, Some(&mut event));
    }

    pub fn wait(&self) -> RawFd {
        let mut event = [EpollEvent::empty()];

        let count = epoll_wait(self.epoll_fd, &mut event, -1).unwrap();
        for c in 0..count {
            let id = event[c].data();

            println!("recv {}", id);
            // match self.event_map.get(id) {
            // 	Some(handler) => handler.exec(),
            // 	None => println!("No handler");
            // }
        }


        return 0;
    }
}
