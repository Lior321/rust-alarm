pub mod args;
pub mod types;

use messages::messages::{AddTimerMsg, Serializeable};
use std::fs::OpenOptions;
use std::io::Write;

static FIFO_PATH: &str = "/tmp/alarm-server.fifo";

fn main() {
    let msg = AddTimerMsg {
        duration: 1,
        is_repeat: false,
    };

    let mut pipe = OpenOptions::new()
        .read(false) // O_RDONLY
        .write(true) // + O_WRONLY = O_RDWR (The EOF trick!)
        .open(FIFO_PATH)
        .expect("Failed to open FIFO");

    let msg = msg.serialize();
    pipe.write_all(&*msg)
        .expect("TODO: panic message");
    println!("Hello, world!");
}
