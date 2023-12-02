use crate::message::Message;

pub trait IPlayer {
    fn send_message(&mut self, m: Message);
}
