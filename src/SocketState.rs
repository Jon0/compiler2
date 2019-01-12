use std::os::unix::io::RawFd;

use nix::unistd::{read, write};

pub struct SocketState {
    fd: RawFd
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


}
