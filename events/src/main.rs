use crate::event::{EventHandle, IEvent};
use crate::event_runner::EventManager;
use std::thread;
use std::time::Duration;
use crate::timeout_event::count_on_interval;

pub mod event;
pub mod event_runner;
pub mod timeout_event;

pub struct Event {
    i: u32,
    id: u32,
}

impl IEvent for Event {
    fn handle(&mut self) -> Option<bool> {
        println!("handle event id: {}, index {}", self.id, self.i);
        self.i += 1;
        Some(true)
    }
}

fn main() {
    let event_manager = EventManager::new();
    event_manager.start();

    // Spawn several producer threads
    let e = EventHandle::new(Event { id: 0, i: 0 });
    count_on_interval(event_manager, e, Duration::from_secs(1), Duration::from_secs(1));

    thread::sleep(Duration::from_secs(10));

    println!("Hello, world!");
}
