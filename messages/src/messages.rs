pub trait Serializeable {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserializeable {
    fn deserialize(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized;
}

pub trait IsMessage {
    fn to_message(&self) -> Message;
}

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

pub fn deserialize(buffer: &[u8]) -> Option<Message> {
    match OpCode::try_from(buffer[0]) {
        Ok(OpCode::AddTimer) => match AddTimerMsg::deserialize(buffer) {
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
