use events::event::EventHandler;
use std::process::Command;

pub struct Timer {
    message: String,
}

impl Timer {
    pub fn new(txt: String) -> Self {
        Self { message: txt }
    }
}

impl EventHandler for Timer {
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

impl Clone for Timer {
    fn clone(&self) -> Self {
        Timer {
            message: self.message.clone(),
        }
    }
}
