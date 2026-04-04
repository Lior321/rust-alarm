use esm::epoll_event::EpollEvent;
use events::event_runner::EventManager;
use std::fs::File;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::sync::{Arc};
use std::time::Duration;
use events::event::EventHandle;
use events::timeout_event::count_once;
use crate::timer::Timer;

pub(crate) struct PipeHandler {
    file: File,
    timeout_handler: Arc<EventManager>,
}

impl PipeHandler {
    pub fn new(fd: RawFd) -> Self {
        let handler = Self {
            file: unsafe { File::from_raw_fd(fd) },
            timeout_handler: EventManager::new(),
        };

        handler.timeout_handler.start();
        handler
    }

    pub fn get_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl EpollEvent for PipeHandler {
    fn handle(&mut self) -> Option<bool> {
        println!("handle pipe");
        let mut buffer = vec![0; 10];
        self.file.read(&mut buffer).expect("failed to read pipe");

        println!("buffer: {:?}", buffer);
        // let msg = messages::messages::deserialize(&buffer).expect("failed to deserialize message");
        count_once(Arc::clone(&self.timeout_handler), EventHandle::new(Timer::new("test")), Duration::from_secs(1));
        println!("final");
        Some(true)
    }
}
