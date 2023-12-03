use crate::message::Message;
use crate::cards::Card;

pub trait IPlayer {
    fn send_message(&mut self, m: Message);
    fn get_cards(&mut self) -> Vec<Card>;
    fn get_playing_cards(&mut self) -> Vec<Card>;
    fn set_cards(&mut self, cards: Vec<Card>);
}
