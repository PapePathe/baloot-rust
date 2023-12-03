use crate::cards::Card;
use crate::iplayers::IPlayer;
use crate::message::Message;

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
    fn send_message(&mut self, m: Message) {
        match m {
            Message::TakingCards(cards) => {
                self.cards = cards;
            }
            Message::PlayingCards(cards) => {
                self.playing_cards = cards;
            }
            Message::Deck(deck) => {
                println!("Wip: send deck through the network and wait for human reaction")
            }
            _ => {
                println!("Was no able to match message")
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
