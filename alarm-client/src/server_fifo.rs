use alarm_common::messages::messages::{serialize, Message};
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct ServerUds {
    socket: UnixStream,
}

impl ServerUds {
    pub fn new(path: &String) -> Result<ServerUds, std::io::Error> {
        Ok(ServerUds {
            socket: UnixStream::connect(path)?,
        })
    }

    pub fn write(&mut self, msg: &Message) -> Result<usize, std::io::Error> {
        let data = serialize(msg);
        Ok(self.socket.write(&data)?)
    }
}
