use crate::timer::Timer;
use esm::epoll_event::EpollEvent;
use events::event::EventHandle;
use events::event_runner::EventManager;
use events::timeout_event::{count_on_interval, count_once};
use messages::messages::AddTimerMsg;
use messages::messages::Message::AddTimer;
use std::fs::File;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::sync::Arc;
use std::time::Duration;

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

    fn handle_add_timer(&self, msg: AddTimerMsg) -> Option<bool> {
        if msg.is_repeat {
            count_on_interval(
                Arc::clone(&self.timeout_handler),
                EventHandle::new(Timer::new("test")),
                Duration::from_secs(msg.duration),
                Duration::from_secs(msg.duration),
            );
        } else {
            count_once(
                Arc::clone(&self.timeout_handler),
                EventHandle::new(Timer::new("test")),
                Duration::from_secs(msg.duration),
            );
        }
        Some(true)
    }
}

impl EpollEvent for PipeHandler {
    fn handle(&mut self) -> Option<bool> {
        println!("handle pipe");
        let mut buffer = vec![0; 10];
        self.file.read(&mut buffer).expect("failed to read pipe");

        println!("buffer: {:?}", buffer);
        match messages::messages::deserialize(&buffer)? {
            AddTimer(msg) => self.handle_add_timer(msg),
        }
    }
}
