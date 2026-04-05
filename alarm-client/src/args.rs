use messages::messages::{AddTimerMsg, IsMessage, Message};
use std::env;

pub fn parse_args() -> Result<Message, &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Too few arguments.");
    }

    match args[1].as_str() {
        "--add-timer" => {
            if args.len() != 3 && args.len() != 4 {
                return Err("--add-timer requires duration and optional is_repeat arguments only");
            }

            Ok(AddTimerMsg {
                duration: args[2].parse().expect("Duration must be an integer"),
                is_repeat: match args.len() {
                    3 => false,
                    4 => args[3].parse().expect("Is repeat must be true/false"),
                    _ => panic!("Impossible code reached"),
                },
            }
            .to_message())
        }
        _ => Err("Invalid command received"),
    }
}
