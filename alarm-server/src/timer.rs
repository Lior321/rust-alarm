use events::event::IEvent;
use std::process::Command;

pub struct Timer {
    message: String,
}

impl Timer {
    pub fn new(txt: String) -> Self {
        Self { message: txt }
    }
}

impl IEvent for Timer {
    fn handle(&mut self) -> bool {
        match Command::new("xcowsay").arg(self.message.as_str()).spawn() {
            Ok(mut child) => match child.wait() {
                Ok(_) => true,
                Err(err) => {
                    eprintln!("xcowsay failed: {}", err);
                    false
                }
            },
            Err(err) => {
                eprintln!("Failed to run xcowsay: {}", err);
                false
            }
        }
    }
}
