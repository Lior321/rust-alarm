use crate::timer::Timer;
use esm::epoll_event::EpollEvent;
use events::event::EventHandle;
use events::event_runner::EventManager;
use events::timeout_event::{count_on_interval, count_once};
use alarm_common::messages::add_timer::AddTimerMsg;
use alarm_common::messages::messages::Message::AddTimer;
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::net::UnixDatagram;
use std::sync::Arc;
use std::time::Duration;

pub(crate) struct UdsHandler {
    uds: UnixDatagram,
    timeout_handler: Arc<EventManager>,
}

impl UdsHandler {
    pub fn new(uds: UnixDatagram) -> Self {
        let handler = Self {
            uds,
            timeout_handler: EventManager::new(),
        };

        handler.timeout_handler.start();
        handler
    }

    pub fn get_fd(&self) -> RawFd {
        self.uds.as_raw_fd()
    }

    fn handle_add_timer(&self, msg: AddTimerMsg) -> bool {
        if msg.is_repeat {
            count_on_interval(
                Arc::clone(&self.timeout_handler),
                EventHandle::new(Timer::new(msg.message.clone())),
                Duration::from_secs(msg.duration),
                Duration::from_secs(msg.duration),
            );
        } else {
            count_once(
                Arc::clone(&self.timeout_handler),
                EventHandle::new(Timer::new(msg.message.clone())),
                Duration::from_secs(msg.duration),
            );
        }
        true
    }
}

impl EpollEvent for UdsHandler {
    fn handle(&mut self) -> bool {
        println!("handle pipe");
        let mut buffer = vec![0; 1024];
        let (_size, _peer) = self
            .uds
            .recv_from(&mut buffer)
            .expect("failed to read pipe");

        println!("buffer: {:?}", buffer);
        match alarm_common::messages::messages::deserialize(&buffer) {
            Some(AddTimer(msg)) => self.handle_add_timer(msg),
            None => false,
        }
    }
}
