pub mod args;
pub mod server_fifo;

use crate::args::parse_args;
use crate::server_fifo::ServerUds;

static FIFO_PATH: &str = "/tmp/alarm-server.fifo";

fn main() {
    let msg = match parse_args() {
        Ok(msg) => msg,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };


    let mut pipe = ServerUds::new(&FIFO_PATH.to_string()).expect("Failed to create server fifo");

    pipe.write(&msg).expect("Failed to write message");
}
