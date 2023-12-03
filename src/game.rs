use crate::cards::{Card, CardColor, CardFamily};
use crate::gametake::GameTake;
use crate::gametake::Take;
use crate::iplayers::IPlayer;
use crate::message::Message;
use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Game<T: IPlayer> {
    pub players: Vec<T>,
    pub cards: Vec<Card>,
    pub take: GameTake,
    pub takes: Vec<GameTake>,
}

impl<T> Default for Game<T>
where
    T: IPlayer,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Game<T>
where
    T: IPlayer,
{
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
        Self {
            players: vec![],
            cards: Self::shuffle_cards(),
            take: GameTake::Skip(Take::SKIP),
            takes: vec![],
        }
    }

    pub fn add_player(&mut self, mut player: T) {
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
    }

    pub fn add_take(&mut self, _: T, take: GameTake) {
        if take != GameTake::Skip(Take::SKIP) {
            self.take = take.clone();
        }

        self.takes.push(take.clone());

        match take {
            GameTake::Tout(_) => {
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
            }
            _ => {
                if self.takes.len() == 4 {
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
                }
            }
        }
    }
}
