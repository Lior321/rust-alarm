use crate::message_traits::{Deserializeable, IsMessage, Serializeable};
use crate::messages::{Message, OpCode};

pub struct AddTimerMsg {
    pub duration: u64,
    pub is_repeat: bool,
}

impl Serializeable for AddTimerMsg {
    fn serialize(&self) -> Vec<u8> {
        let mut vector = vec![OpCode::AddTimer as u8];
        vector.extend(self.duration.to_le_bytes());
        vector.push(self.is_repeat as u8);

        vector
    }
}

impl Deserializeable for AddTimerMsg {
    fn deserialize(buffer: &[u8]) -> Option<Self> {
        if size_of::<AddTimerMsg>() < buffer.len() {
            return None;
        }

        Some(AddTimerMsg {
            duration: u64::from_le_bytes(buffer[1..9].try_into().unwrap()),
            is_repeat: buffer[9] != 0,
        })
    }
}

impl Clone for AddTimerMsg {
    fn clone(&self) -> Self {
        AddTimerMsg {
            duration: self.duration,
            is_repeat: self.is_repeat,
        }
    }
}

impl IsMessage for AddTimerMsg {
    fn to_message(&self) -> Message {
        Message::AddTimer(self.clone())
    }
}
