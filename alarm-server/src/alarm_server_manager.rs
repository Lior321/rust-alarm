use crate::uds_handler::UdsHandler;
use alarm_common::alarm_error::AlarmError;
use esm::esm::ESM;
use std::fs::{exists, remove_file};
use std::os::unix::net::UnixDatagram;

static FIFO_PATH: &str = "/tmp/server.sock";

pub(crate) struct AlarmServerManager {
    esm: ESM<UdsHandler>,
}

impl AlarmServerManager {
    pub fn new() -> Result<AlarmServerManager, AlarmError> {
        if exists(FIFO_PATH).expect("Failed to check for existence") {
            remove_file(FIFO_PATH).expect("Failed to remove file");
        }

        let sock = UdsHandler::new(UnixDatagram::bind(FIFO_PATH)?);
        let mut alarm_manager = AlarmServerManager { esm: ESM::new()? };

        match alarm_manager.esm.add_event(sock.get_fd(), sock) {
            Ok(_) => Ok(alarm_manager),
            Err(err) => Err(AlarmError::from(err)),
        }
    }

    pub fn run(&mut self) -> Result<(), AlarmError> {
        self.esm.dispatch_indefinitely()?;
        Ok(())
    }
}
