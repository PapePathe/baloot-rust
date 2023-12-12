use crate::cards::Card;
use crate::cards::CardColor;
use crate::cards::CardFamily;

#[derive(Debug, PartialEq, Clone)]
pub struct Take(pub u8);

impl Take {
    pub const TOUT: Take = Take(6);
    pub const CENT: Take = Take(5);
    pub const SPADE: Take = Take(4);
    pub const HEART: Take = Take(3);
    pub const DIAMOND: Take = Take(2);
    pub const CLUBS: Take = Take(1);
    pub const SKIP: Take = Take(0);

    pub fn takes() -> Vec<Take> {
        vec![
            Take::CLUBS,
            Take::DIAMOND,
            Take::HEART,
            Take::SPADE,
            Take::CENT,
            Take::TOUT,
        ]
    }

    fn has_two_of(self, family: CardFamily, cards: Vec<Card>) -> bool {
        let items: Vec<&Card> = cards
            .iter()
            .filter(|c| c.family == family)
            .collect::<Vec<_>>();

        items.len() >= 2
    }

    fn has_jack_and_nine(self, color: CardColor, cards: Vec<Card>) -> bool {
        let jack = Card::new(color, CardFamily::JACK);
        let nine = Card::new(color, CardFamily::NINE);

        if cards.contains(&jack) && cards.contains(&nine) {
            return true;
        }

        false
    }

    pub fn evaluate(self, cards: Vec<Card>) -> (bool, GameTake, u8) {
        match self.0 {
            6 => {
                if self.has_two_of(CardFamily::JACK, cards) {
                    return (true, GameTake::Tout(Take::TOUT), 0);
                }

                (false, GameTake::Skip(Take::SKIP), 0)
            }
            5 => {
                if self.has_two_of(CardFamily::ACE, cards) {
                    return (true, GameTake::Cent(Take::CENT), 0);
                }

                (false, GameTake::Skip(Take::SKIP), 0)
            }
            4 => {
                if self.has_jack_and_nine(CardColor::SPADES, cards) {
                    return (true, GameTake::Spade(Take::SPADE), 0);
                }

                (false, GameTake::Skip(Take::SKIP), 0)
            }
            3 => {
                if self.has_jack_and_nine(CardColor::HEARTS, cards) {
                    return (true, GameTake::Heart(Take::HEART), 0);
                }

                (false, GameTake::Skip(Take::SKIP), 0)
            }
            2 => {
                if self.has_jack_and_nine(CardColor::DIAMONDS, cards) {
                    return (true, GameTake::Diamond(Take::DIAMOND), 0);
                }

                (false, GameTake::Skip(Take::SKIP), 0)
            }
            1 => {
                if self.has_jack_and_nine(CardColor::CLUBS, cards) {
                    return (true, GameTake::Club(Take::CLUBS), 0);
                }

                (false, GameTake::Skip(Take::SKIP), 0)
            }
            _ => (false, GameTake::Skip(Take::SKIP), 0),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameTake {
    Tout(Take),
    Cent(Take),
    Spade(Take),
    Heart(Take),
    Diamond(Take),
    Club(Take),
    Skip(Take),
}
