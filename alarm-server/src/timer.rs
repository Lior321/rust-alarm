use events::event::IEvent;
use std::process::Command;

pub struct Timer {
    message: &'static str
}

impl Timer {
    pub fn new(txt: &'static str) -> Self {
        Self { message: txt }
    }
}

impl IEvent for Timer {
    fn handle(&mut self) -> Option<bool> {
        Command::new("xcowsay").arg(self.message).spawn().unwrap().wait().unwrap();

        Some(true)
    }
}
