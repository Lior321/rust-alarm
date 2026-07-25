pub mod args;
pub mod server_fifo;

use alarm_common::messages::messages::{Message, serialize};

use crate::args::parse_args;
use std::os::unix::net::UnixDatagram;

static FIFO_PATH: &str = "/tmp/server.sock";

fn main() {
    let msg: Message = match parse_args() {
        Ok(msg) => msg,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    let socket = UnixDatagram::unbound().expect("Failed to create a socket");

    // 2. "Connect" to the bound path (sets the default destination)
    socket
        .connect(&FIFO_PATH)
        .expect("Failed to connect to the server's UDS");

    let data = serialize(&msg);
    socket.send(&data[0..]).expect("Failed to write message");
}
