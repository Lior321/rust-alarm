use crate::messages::messages::Message;

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
