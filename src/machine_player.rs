use crate::cards::Card;
use crate::cards::CardColor;
use crate::cards::CardFamily;
use crate::gametake::GameTake;
use crate::gametake::Take;
use crate::iplayers::IPlayer;
use crate::message::Message;
use crate::message::PlayerMessage;

use std::collections::HashMap;
use std::fmt::Debug;

use crossbeam_channel::Sender;
use paris::Logger;

#[derive(Clone, Debug)]
pub struct MachinePlayer {
    cards: Vec<Card>,
    playing_cards: Vec<Card>,
    deck: Vec<Card>,
}

impl MachinePlayer {
    fn get_cards_tree(self, playing_cards: bool) -> HashMap<CardColor, Vec<CardFamily>> {
        let mut hm: HashMap<CardColor, Vec<CardFamily>> = HashMap::new();
        let cards: Vec<Card>;
        if playing_cards {
            cards = self.playing_cards;
        } else {
            cards = self.cards
        }

        for c in cards {
            hm.entry(c.color)
                .and_modify(|h: &mut Vec<CardFamily>| h.push(c.family))
                .or_insert(vec![c.family]);
        }

        hm
    }

    pub fn new(cards: Vec<Card>) -> Self {
        Self {
            cards,
            playing_cards: vec![],
            deck: vec![],
        }
    }
}

impl IPlayer for MachinePlayer {
    fn set_taking_channel(&mut self, _: Sender<PlayerMessage>) {}

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
        let mut log = Logger::new();

        match m {
            Message::TakingCards(cards) => {
                self.cards = cards.clone();
            }
            Message::PlayingCards(cards) => {
                self.playing_cards = cards;
                log.info("Machine Player:")
                    .indent(1)
                    .success("Received playing hand")
                    .indent(2)
                    .info(format!("cards: {:?}", self.clone().get_cards_tree(true)));
            }
            Message::Deck(deck) => {
                log.info("Deck updated")
                    .indent(1)
                    .success(format!("cards: {:?}", deck));
                self.deck = deck;
            }
            Message::PleaseTake(c) => {
                log.info("Machine player")
                    .indent(1)
                    .info("Received please take message")
                    .indent(2)
                    .info(format!("{:?}", self.clone().get_cards_tree(false)));
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

                if !takes.is_empty() {
                    let _ = c.send(PlayerMessage::SetPlayerTake(takes[0].1.clone()));
                } else {
                    let _ = c.send(PlayerMessage::SetPlayerTake(GameTake::Skip(Take::SKIP)));
                }
            }
            Message::PleasePlay(channel) => {
                log.info("Machine Player")
                    .indent(1)
                    .success("Current hand before play")
                    .indent(2)
                    .success(format!(": {:?}", self.clone().get_cards_tree(true)))
                    .success(format!("Current Deck: {:?}", self.deck));

                let tree = self.clone().get_cards_tree(true);

                if self.deck.is_empty() {
                    log.info("Machine player")
                        .indent(1)
                        .warn("Going to select best card to play");
                } else {
                    if tree.contains_key(&self.deck[0].color) {
                        log.info(format!("Player has card of family {}", self.deck[0].color));
                    } else {
                        log.info("Machine player")
                            .indent(1)
                            .warn("Going to select least valuable card to play");
                    }
                }

                let c = self.playing_cards.pop();

                if let Some(c) = c {
                    let _ = channel.send(PlayerMessage::PlayCard(c));
                }
            }
        }
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new(cards: Vec<Card>) -> Self {
        Self { cards }
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
