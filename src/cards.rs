use crate::gametake::{GameTake, Take};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub family: CardFamily,
    pub color: CardColor,
}

impl Card {
    pub fn new(c: CardColor, f: CardFamily) -> Self {
        Self {
            color: c,
            family: f,
        }
    }

    pub fn is_ace(self) -> bool {
        matches!(self.family, CardFamily::ACE)
    }
    pub fn is_nine(self) -> bool {
        matches!(self.family, CardFamily::NINE)
    }
    pub fn is_jack(self) -> bool {
        matches!(self.family, CardFamily::JACK)
    }

    pub fn is_ten(self) -> bool {
        matches!(self.family, CardFamily::TEN)
    }

    pub fn evaluate_for_win(self, take: Take) -> u8 {
        match self.family {
            CardFamily::JACK => match take.0 {
                6 => 20,
                4 => match self.color {
                    CardColor::SPADES => 20,
                    _ => 3,
                },
                3 => match self.color {
                    CardColor::HEARTS => 20,
                    _ => 3,
                },
                2 => match self.color {
                    CardColor::DIAMONDS => 20,
                    _ => 3,
                },
                1 => match self.color {
                    CardColor::CLUBS => 20,
                    _ => 3,
                },
                _ => 3,
            },
            CardFamily::NINE => match take.0 {
                6 => 14,
                _ => 1,
                4 => match self.color {
                    CardColor::SPADES => 14,
                    _ => 1,
                },
                3 => match self.color {
                    CardColor::HEARTS => 14,
                    _ => 1,
                },
                2 => match self.color {
                    CardColor::DIAMONDS => 14,
                    _ => 1,
                },
                1 => match self.color {
                    CardColor::CLUBS => 14,
                    _ => 1,
                },
            },

            CardFamily::ACE => 11,
            CardFamily::TEN => 10,
            CardFamily::KING => 5,
            CardFamily::QUEEN => 4,
            CardFamily::HEIGHT => 2,
            CardFamily::SEVEN => 0,
        }
    }

    pub fn evaluate_for_take(self, take: GameTake) -> u8 {
        match self.family {
            CardFamily::JACK => match take {
                GameTake::Tout(_) => 20,
                GameTake::Cent(_) => 2,
                GameTake::Spade(_) => match self.color {
                    CardColor::SPADES => 20,
                    _ => 2,
                },
                GameTake::Club(_) => match self.color {
                    CardColor::CLUBS => 20,
                    _ => 2,
                },
                GameTake::Diamond(_) => match self.color {
                    CardColor::DIAMONDS => 20,
                    _ => 2,
                },
                GameTake::Heart(_) => match self.color {
                    CardColor::HEARTS => 20,
                    _ => 2,
                },
                GameTake::Skip(_) => todo!(),
            },
            CardFamily::NINE => match take {
                GameTake::Tout(_) => 14,
                GameTake::Cent(_) => 0,
                GameTake::Spade(_) => match self.color {
                    CardColor::SPADES => 14,
                    _ => 0,
                },
                GameTake::Club(_) => match self.color {
                    CardColor::CLUBS => 14,
                    _ => 0,
                },
                GameTake::Heart(_) => match self.color {
                    CardColor::HEARTS => 14,
                    _ => 0,
                },
                GameTake::Diamond(_) => match self.color {
                    CardColor::DIAMONDS => 14,
                    _ => 0,
                },
                GameTake::Skip(_) => todo!(),
            },
            CardFamily::ACE => 11,
            CardFamily::TEN => 10,
            CardFamily::KING => 4,
            CardFamily::QUEEN => 3,
            CardFamily::HEIGHT => 0,
            CardFamily::SEVEN => 0,
        }
    }
}

#[derive(Eq, Hash, Debug, PartialEq, Clone, Copy)]
pub enum CardColor {
    DIAMONDS,
    HEARTS,
    SPADES,
    CLUBS,
}

impl std::fmt::Display for CardColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            CardColor::CLUBS => write!(f, "Clubs"),
            CardColor::DIAMONDS => write!(f, "Diamonds"),
            CardColor::HEARTS => write!(f, "hearts"),
            CardColor::SPADES => write!(f, "spades"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardFamily {
    ACE,
    SEVEN,
    HEIGHT,
    NINE,
    TEN,
    JACK,
    KING,
    QUEEN,
}

impl std::fmt::Display for CardFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            CardFamily::ACE => write!(f, "A"),
            CardFamily::HEIGHT => write!(f, "8"),
            CardFamily::JACK => write!(f, "V"),
            CardFamily::KING => write!(f, "K"),
            CardFamily::NINE => write!(f, "9"),
            CardFamily::QUEEN => write!(f, "D"),
            CardFamily::SEVEN => write!(f, "7"),
            CardFamily::TEN => write!(f, "10"),
        }
    }
} //   static AsPique: Card = Card::new(CardColor::SPADES, CardFamily::ACE);
