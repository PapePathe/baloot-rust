use crate::cards::Card;
use crate::iplayers::IPlayer;
use crate::message::Message;
use crate::message::PlayerMessage;

use crossbeam_channel::Sender;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    cards: Vec<Card>,
    playing_cards: Vec<Card>,
}

impl Player {
    pub fn new(name: String, cards: Vec<Card>) -> Self {
        Self {
            name,
            cards,
            playing_cards: vec![],
        }
    }
}

impl IPlayer for Player {
    fn set_taking_channel(&mut self, _: Sender<PlayerMessage>) {}

    fn send_message(&mut self, m: Message) {
        match m {
            Message::TakingCards(cards) => {
                self.cards = cards;
            }
            Message::PlayingCards(cards) => {
                self.playing_cards = cards;
            }
            Message::Deck(_) => {
                println!("Wip: send deck through the network and wait for human reaction")
            }
            Message::PleaseTake(_) => {
                println!("\n\n Your cards are: {:?}", self.cards);
                println!("Please choose of the takes below")
            }
            Message::PleasePlay(_) => {
                println!("Please choose of the takes below");
            }
        }
    }
    fn get_playing_cards(&mut self) -> Vec<Card> {
        self.playing_cards.clone()
    }
    fn get_cards(&mut self) -> Vec<Card> {
        self.cards.clone()
    }
    fn set_cards(&mut self, cards: Vec<Card>) {
        self.cards = cards
    }
}

#[cfg(test)]
mod player_test {
    use crate::players::*;

    #[test]
    fn test_new_player() {
        let p = Player::new(String::from("pathe"), vec![]);
        assert_eq!(p.name, "pathe");
        assert_eq!(p.cards.len(), 0);
    }
}
