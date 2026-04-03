use crate::pipe_handler::PipeHandler;
use esm::esm::ESM;
use std::fs::{exists, remove_file, OpenOptions, Permissions};
use std::os::fd::IntoRawFd;
use std::os::unix::fs::{mkfifo, PermissionsExt};

static FIFO_PATH: &str = "/tmp/alarm-server.fifo";

pub(crate) struct AlarmServerManager {
    esm: ESM,
}

impl AlarmServerManager {
    pub fn new() -> Option<AlarmServerManager> {
        if exists(FIFO_PATH).expect("Failed to check for existence") {
            remove_file(FIFO_PATH).expect("Failed to remove file");
        }

        if mkfifo(FIFO_PATH, Permissions::from_mode(0o622)).is_err() {
            return None;
        }

        let pipe_handler = PipeHandler::new(
            OpenOptions::new()
                .read(true) // O_RDONLY
                .write(true) // + O_WRONLY = O_RDWR (The EOF trick!)
                .open(FIFO_PATH)
                .expect("Failed to open FIFO")
                .into_raw_fd(),
        );

        let esm = ESM::new();
        if esm.is_err() {
            return None;
        }

        let mut alarm_manager = AlarmServerManager { esm: esm.unwrap() };
        match alarm_manager
            .esm
            .add_event(pipe_handler.get_fd(), Box::new(pipe_handler))
        {
            true => Some(alarm_manager),
            false => None,
        }
    }

    pub fn run(&mut self) {
        self.esm.dispatch_indefinitely()
    }
}
