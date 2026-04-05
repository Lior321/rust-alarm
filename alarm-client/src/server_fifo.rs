use std::fs::{File, OpenOptions};
use std::io::Write;
use messages::messages::{serialize, Message};

pub struct ServerFifo {
    pipe: File,
}

impl ServerFifo {
    pub fn new(path: &String) -> Result<ServerFifo, std::io::Error> {
        let pipe = OpenOptions::new()
            .read(false) // O_RDONLY
            .write(true) // + O_WRONLY = O_RDWR (The EOF trick!)
            .open(path)?;
        Ok(ServerFifo { pipe })
    }

    pub fn write(&mut self, msg: &Message) -> Result<(), std::io::Error> {
        let data = serialize(msg);
        self.pipe.write_all(&data)
    }
}
