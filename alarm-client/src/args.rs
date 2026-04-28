use alarm_common::messages::add_timer::AddTimerMsg;
use alarm_common::messages::message_traits::IsMessage;
use alarm_common::messages::messages::Message;
use std::env;

pub fn parse_args() -> Result<Message, &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Too few arguments.");
    }

    match args[1].as_str() {
        "--add-timer" => {
            if args.len() != 4 && args.len() != 5 {
                return Err("--add-timer requires duration and optional is_repeat arguments only");
            }

            Ok(AddTimerMsg {
                duration: args[2].parse().expect("Duration must be an integer"),
                message: args[3].clone(),
                is_repeat: match args.len() {
                    4 => false,
                    5 => args[4].parse().expect("Is repeat must be true/false"),
                    _ => panic!("Impossible code reached"),
                },
            }
            .to_message())
        }
        _ => Err("Invalid command received"),
    }
}
