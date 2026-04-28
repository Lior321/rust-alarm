use crate::uds_handler::UdsHandler;
use esm::esm::ESM;
use alarm_common::alarm_error::AlarmError;
use std::fs::{exists, remove_file};
use std::os::unix::net::UnixDatagram;

static FIFO_PATH: &str = "/tmp/alarm-server.fifo";

pub(crate) struct AlarmServerManager {
    esm: ESM,
}

impl AlarmServerManager {
    pub fn new() -> Result<AlarmServerManager, AlarmError> {
        if exists(FIFO_PATH).expect("Failed to check for existence") {
            remove_file(FIFO_PATH).expect("Failed to remove file");
        }

        let sock = UdsHandler::new(UnixDatagram::bind("/tmp/server.sock")?);
        let esm = ESM::new()?;

        let mut alarm_manager = AlarmServerManager { esm };
        match alarm_manager.esm.add_event(sock.get_fd(), Box::new(sock)) {
            Ok(_) => Ok(alarm_manager),
            Err(err) => Err(AlarmError::from(err)),
        }
    }

    pub fn run(&mut self) -> Result<(), AlarmError> {
        self.esm.dispatch_indefinitely()?;
        Ok(())
    }
}
