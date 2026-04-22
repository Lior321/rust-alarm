use crate::add_timer::AddTimerMsg;
use crate::message_traits::{Deserializeable, Serializeable};

#[repr(u8)]
pub enum OpCode {
    AddTimer,
    RemoveTimer,
    AddAlarm,
    RemoveAlarm,
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::AddTimer),
            _ => Err("Invalid value for MyEnum"),
        }
    }
}

pub enum Message {
    AddTimer(AddTimerMsg),
}

pub fn deserialize(buffer: &[u8]) -> Option<Message> {
    match OpCode::try_from(buffer[0]) {
        Ok(OpCode::AddTimer) => match AddTimerMsg::deserialize(&buffer[1..]) {
            Some(msg) => Some(Message::AddTimer(msg)),
            None => None,
        },
        Ok(_) => todo!(),
        Err(_) => {
            eprintln!("Unable to deserialize OpCode");
            None
        }
    }
}

pub fn serialize(message: &Message) -> Vec<u8> {
    match message {
        Message::AddTimer(msg) => msg.serialize(),
    }
}
