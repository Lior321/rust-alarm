#![feature(unix_mkfifo)]

use crate::alarm_server_manager::AlarmServerManager;

mod alarm_server_manager;
pub mod pipe_handler;

fn main() {
    let alarm_server_manager = AlarmServerManager::new();

    alarm_server_manager.expect("Failed to create epoll manager").run()
}

