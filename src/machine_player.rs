use crate::cards::Card;
use crate::iplayers::IPlayer;
use crate::message::Message;

use std::fmt::Debug;
use std::fmt::Formatter;

pub struct MachinePlayer {
    cards: Vec<Card>,
    playing_cards: Vec<Card>,
    deck: Vec<Card>,
}

impl Debug for dyn IPlayer + 'static {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", "iplayer")
    }
}

impl MachinePlayer {
    pub fn new() -> Self {
        Self {
            cards: vec![],
            playing_cards: vec![],
            deck: vec![],
        }
    }
}

impl IPlayer for MachinePlayer {
    fn send_message(&mut self, m: Message) {
        match m {
            Message::TakingCards(cards) => {
                self.cards = cards;
            }
            Message::PlayingCards(cards) => {
                self.playing_cards = cards;
            }
            Message::Deck(deck) => {
                self.deck = deck;
            }
            _ => {
                println!("Was no able to match message")
            }
        }
    }
}

#[cfg(test)]
mod machine_player_test {
    use crate::cards::*;
    use crate::iplayers::IPlayer;
    use crate::machine_player::MachinePlayer;
    use crate::message::Message;

    #[test]
    fn test_send_message_for_deck() {
        let mut p = MachinePlayer {
            cards: vec![],
            playing_cards: vec![],
            deck: vec![],
        };

        let hand = vec![Card::new(CardColor::SPADES, CardFamily::ACE)];
        p.send_message(Message::Deck(hand));

        assert_eq!(p.deck, vec![Card::new(CardColor::SPADES, CardFamily::ACE)]);
        assert_eq!(p.cards, vec![]);
        assert_eq!(p.playing_cards, vec![]);
    }

    #[test]
    fn test_send_message_for_playing_cards() {
        let mut p = MachinePlayer {
            cards: vec![],
            playing_cards: vec![],
            deck: vec![],
        };

        let hand = vec![Card::new(CardColor::SPADES, CardFamily::ACE)];
        p.send_message(Message::PlayingCards(hand));

        assert_eq!(
            p.playing_cards,
            vec![Card::new(CardColor::SPADES, CardFamily::ACE)]
        );
        assert_eq!(p.cards, vec![]);
        assert_eq!(p.deck, vec![]);
    }

    #[test]
    fn test_send_message_for_taking_cards() {
        let mut p = MachinePlayer {
            cards: vec![],
            playing_cards: vec![],
            deck: vec![],
        };

        let hand = vec![Card::new(CardColor::SPADES, CardFamily::ACE)];
        p.send_message(Message::TakingCards(hand));

        assert_eq!(p.cards, vec![Card::new(CardColor::SPADES, CardFamily::ACE)]);
        assert_eq!(p.playing_cards, vec![]);
        assert_eq!(p.deck, vec![]);
    }
}
