use crate::cards::{Card, CardColor, CardFamily};
use crate::gametake::GameTake;
use crate::gametake::Take;
use crate::iplayers::IPlayer;
use crate::message::Message;
use crate::message::PlayerMessage;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::fmt::Debug;
use std::fmt::Formatter;

use postage::dispatch;

impl Debug for dyn IPlayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("IPlayer").finish()
    }
}

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Box<dyn IPlayer>>,
    pub cards: Vec<Card>,
    pub take: GameTake,
    pub takes: Vec<GameTake>,
    pub decks: Vec<Vec<Card>>,
    pub current_deck: usize,
    pub ring: Vec<u8>,
    pub started: bool,
    pub finished: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    fn shuffle_cards() -> Vec<Card> {
        let mut cards: Vec<Card> = vec![];
        let colors = vec![
            CardColor::CLUBS,
            CardColor::DIAMONDS,
            CardColor::HEARTS,
            CardColor::SPADES,
        ];
        let families = vec![
            CardFamily::ACE,
            CardFamily::SEVEN,
            CardFamily::HEIGHT,
            CardFamily::NINE,
            CardFamily::TEN,
            CardFamily::JACK,
            CardFamily::KING,
            CardFamily::QUEEN,
        ];

        for color in colors {
            for family in &families {
                cards.push(Card::new(color, *family));
            }
        }
        cards.shuffle(&mut thread_rng());

        cards
    }

    pub fn new() -> Self {
        let disp = dispatch::channel::<PlayerMessage>(8);
        Self {
            players: vec![],
            cards: Self::shuffle_cards(),
            take: GameTake::Skip(Take::SKIP),
            takes: vec![],
            ring: vec![0, 1, 2, 3],
            started: false,
            finished: false,
            decks: vec![],
            current_deck: 0,
        }
    }

    pub fn add_player(&mut self, mut player: Box<dyn IPlayer>) {
        if self.players.len() < 4 {
            let mut pcards: Vec<Card> = vec![];

            for i in 0..5 {
                let c = self.cards[i];
                pcards.push(c);
                if let Some(pos) = self.cards.iter().position(|x| *x == c) {
                    self.cards.remove(pos);
                }
            }
            player.send_message(Message::TakingCards(pcards));
            self.players.push(player);
        }

        if self.players.len() == 4 {
            self.players[0].send_message(Message::PleaseTake);
        }
    }

    pub fn receive_message(&mut self, _m: PlayerMessage) {
        //println!("game {:?} received message {:?} from player", self, m)
    }

    pub fn add_card(&mut self, c: Card) {
        if self.decks.len() == 0 {
            self.decks.push(vec![]);
        }

        self.decks[self.current_deck].push(c);

        println!("current deck {:?}", self.current_deck);
        if self.decks[self.current_deck].len() < 4 {
            self.players[self.decks[self.current_deck].len()].send_message(Message::PleasePlay);
        } else {
            if self.current_deck < 7 {
                self.current_deck += 1;
                self.decks.push(vec![]);
                self.players[0].send_message(Message::PleasePlay);
            } else {
                self.finished = true
            }
        }

        println!("game finished: {}", self.finished);
    }

    pub fn add_take(&mut self, _: u8, take: GameTake) {
        if take != GameTake::Skip(Take::SKIP) {
            self.take = take.clone();
        }

        self.takes.push(take.clone());

        match take {
            GameTake::Tout(_) => {
                self.started = true;
                for p in &mut self.players {
                    let mut pcards = p.get_cards();

                    for _ in 0..3 {
                        let mut c = self.cards.pop();
                        if let Some(v) = c.as_mut() {
                            pcards.push(*v);
                            if let Some(pos) = self.cards.iter().position(|x| *x == *v) {
                                self.cards.remove(pos);
                            }
                        }
                    }

                    p.send_message(Message::PlayingCards(pcards))
                }

                self.players[0].send_message(Message::PleasePlay);
            }
            _ => {
                if self.takes.len() < 4 {
                    self.players[self.takes.len()].send_message(Message::PleaseTake)
                }
                if self.takes.len() == 4 {
                    self.started = true;
                    for p in &mut self.players {
                        let mut pcards = p.get_cards();

                        for _ in 0..3 {
                            let mut c = self.cards.pop();
                            if let Some(v) = c.as_mut() {
                                pcards.push(*v);
                                if let Some(pos) = self.cards.iter().position(|x| *x == *v) {
                                    self.cards.remove(pos);
                                }
                            }
                        }
                        p.send_message(Message::PlayingCards(pcards))
                    }

                    self.players[0].send_message(Message::PleasePlay)
                } else {
                }
            }
        }
    }
}
