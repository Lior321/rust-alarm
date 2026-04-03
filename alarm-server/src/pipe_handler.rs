use esm::epoll_event::EpollEvent;
use std::fs::File;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};

pub(crate) struct PipeHandler {
    file: File,
}

impl PipeHandler {
    pub fn new(fd: RawFd) -> Self {
        Self {
            file: unsafe { File::from_raw_fd(fd) },
        }
    }

    pub fn get_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl EpollEvent for PipeHandler {
    fn handle(&mut self) -> Option<bool> {
        println!("handle pipe");
        let mut buffer = vec![0; 10];

        self.file.read(&mut buffer).unwrap();

        println!("buffer: {:?}", buffer);
        Some(true)
    }
}
