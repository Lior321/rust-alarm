use std::str::from_utf8;

use crate::messages::message_traits::{Deserializeable, IsMessage, Serializeable};
use crate::messages::messages::{Message, OpCode};

pub struct AddTimerMsg {
    pub duration: u64,
    pub is_repeat: bool,
    pub message: String,
}

impl Serializeable for AddTimerMsg {
    fn serialize(&self) -> Vec<u8> {
        let mut vector = vec![OpCode::AddTimer as u8];
        vector.extend(self.duration.to_le_bytes());
        vector.push(self.is_repeat as u8);
        vector.extend(self.message.as_bytes());

        vector
    }
}

impl Deserializeable for AddTimerMsg {
    fn deserialize(buffer: &[u8]) -> Option<Self> {
        let duration = u64::from_le_bytes(
            match buffer[0..8].try_into() {
                Ok(bytes) => bytes,
                Err(_) => return None,
            }
        );
        let message = match from_utf8(&buffer[9..]) {
            Ok(v) => v,
            Err(_) => return None,
        }.trim_matches(char::from(0));


        Some(AddTimerMsg {
            duration,
            is_repeat: buffer[9] != 0,
            message: message.to_string(),
        })
    }
}

impl Clone for AddTimerMsg {
    fn clone(&self) -> Self {
        AddTimerMsg {
            duration: self.duration,
            is_repeat: self.is_repeat,
            message: self.message.clone(),
        }
    }
}

impl IsMessage for AddTimerMsg {
    fn to_message(&self) -> Message {
        Message::AddTimer(self.clone())
    }
}
