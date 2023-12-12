use crate::cards::Card;
use crate::cards::CardColor;
use crate::cards::CardFamily;
use crate::gametake::GameTake;
use crate::gametake::Take;
use crate::iplayers::IPlayer;
use crate::message::Message;
use crate::message::PlayerMessage;
use tokio::join;

use std::collections::HashMap;
use std::fmt::Debug;

use postage::dispatch::Sender;
use postage::prelude::Sink;

#[derive(Clone, Debug)]
pub struct MachinePlayer {
    cards: Vec<Card>,
    playing_cards: Vec<Card>,
    deck: Vec<Card>,
    sender: crossbeam_channel::Sender<PlayerMessage>,
}

impl MachinePlayer {
    fn get_cards_tree(self) -> HashMap<CardColor, Vec<CardFamily>> {
        let mut hm: HashMap<CardColor, Vec<CardFamily>> = HashMap::new();
        for c in self.cards {
            hm.entry(c.color)
                .and_modify(|h: &mut Vec<CardFamily>| h.push(c.family))
                .or_insert(vec![c.family]);
        }

        return hm;
    }

    pub fn new(cards: Vec<Card>, sender: crossbeam_channel::Sender<PlayerMessage>) -> Self {
        Self {
            cards,
            playing_cards: vec![],
            deck: vec![],
            sender: sender,
        }
    }
}

impl IPlayer for MachinePlayer {
    fn set_taking_channel(&mut self, sender: Sender<PlayerMessage>) {}

    fn get_cards(&mut self) -> Vec<Card> {
        self.cards.clone()
    }

    fn get_playing_cards(&mut self) -> Vec<Card> {
        self.playing_cards.clone()
    }

    fn set_cards(&mut self, hand: Vec<Card>) {
        self.cards = hand
    }

    fn send_message(&mut self, m: Message) {
        match m {
            Message::TakingCards(cards) => {
                self.cards = cards.clone();
            }
            Message::PlayingCards(cards) => {
                println!("Received playing cards {:?}, len: {}", cards, cards.len());
                println!("");
                self.playing_cards = cards;
            }
            Message::Deck(deck) => {
                self.deck = deck;
            }
            Message::PleaseTake => {
                println!("{:?}", self.clone().get_cards_tree());
                let mut takes: Vec<(bool, GameTake, u8)> = vec![];
                for t in Take::takes() {
                    let result = t.evaluate(self.get_cards());
                    match result {
                        (true, _, _) => {
                            takes.push(result);
                        }
                        (false, _, _) => {}
                    }
                }

                if takes.len() > 0 {
                    let _ = self
                        .sender
                        .send(PlayerMessage::SetPlayerTake(takes[0].1.clone()));
                } else {
                    let _ = self
                        .sender
                        .send(PlayerMessage::SetPlayerTake(GameTake::Skip(Take::SKIP)));
                }
            }
            Message::PleasePlay => {
                let c = self.playing_cards.pop();
                println!("remaining cards {:?}", self.playing_cards);
                if let Some(c) = c {
                    let _ = self.sender.send(PlayerMessage::PlayCard(c));
                }
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
