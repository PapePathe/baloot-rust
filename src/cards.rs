use crate::gametake::GameTake;

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
        false
    }
    pub fn is_jack(self) -> bool {
        matches!(self.family, CardFamily::JACK)
    }
    pub fn is_ten(self) -> bool {
        matches!(self.family, CardFamily::TEN)
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

#[cfg(test)]
mod cards_test {
    use crate::cards::*;
    use crate::gametake::Take;

    #[test]
    fn test_evaluate_for_take_cent() {
        let c = Card::new(CardColor::SPADES, CardFamily::ACE);
        assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 11);

        let c = Card::new(CardColor::SPADES, CardFamily::TEN);
        assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 10);

        let c = Card::new(CardColor::SPADES, CardFamily::KING);
        assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 4);

        let c = Card::new(CardColor::SPADES, CardFamily::QUEEN);
        assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 3);

        let c = Card::new(CardColor::SPADES, CardFamily::JACK);
        assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 2);

        let c = Card::new(CardColor::SPADES, CardFamily::NINE);
        assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 0);

        let c = Card::new(CardColor::SPADES, CardFamily::HEIGHT);
        assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 0);

        let c = Card::new(CardColor::SPADES, CardFamily::SEVEN);
        assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 0);
    }

    #[test]
    fn test_evaluate_for_take_spades() {
        let c = Card::new(CardColor::SPADES, CardFamily::JACK);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 20);

        let c = Card::new(CardColor::CLUBS, CardFamily::JACK);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 2);

        let c = Card::new(CardColor::HEARTS, CardFamily::JACK);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 2);

        let c = Card::new(CardColor::DIAMONDS, CardFamily::JACK);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 2);

        let c = Card::new(CardColor::SPADES, CardFamily::NINE);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 14);

        let c = Card::new(CardColor::CLUBS, CardFamily::NINE);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);

        let c = Card::new(CardColor::HEARTS, CardFamily::NINE);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);

        let c = Card::new(CardColor::DIAMONDS, CardFamily::NINE);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);

        let c = Card::new(CardColor::SPADES, CardFamily::ACE);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 11);

        let c = Card::new(CardColor::SPADES, CardFamily::TEN);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 10);

        let c = Card::new(CardColor::SPADES, CardFamily::KING);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 4);

        let c = Card::new(CardColor::SPADES, CardFamily::QUEEN);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 3);

        let c = Card::new(CardColor::SPADES, CardFamily::HEIGHT);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);

        let c = Card::new(CardColor::SPADES, CardFamily::SEVEN);
        assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);
    }
    #[test]
    fn test_evaluate_for_take_tout() {
        let c = Card::new(CardColor::SPADES, CardFamily::JACK);
        assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 20);

        let c = Card::new(CardColor::SPADES, CardFamily::NINE);
        assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 14);

        let c = Card::new(CardColor::SPADES, CardFamily::ACE);
        assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 11);

        let c = Card::new(CardColor::SPADES, CardFamily::TEN);
        assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 10);

        let c = Card::new(CardColor::SPADES, CardFamily::KING);
        assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 4);

        let c = Card::new(CardColor::SPADES, CardFamily::QUEEN);
        assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 3);

        let c = Card::new(CardColor::SPADES, CardFamily::HEIGHT);
        assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 0);

        let c = Card::new(CardColor::SPADES, CardFamily::SEVEN);
        assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 0);
    }

    #[test]
    fn test_new_card() {
        let c = Card::new(CardColor::SPADES, CardFamily::JACK);

        assert_eq!(CardColor::SPADES, c.color);
        assert_eq!(CardFamily::JACK, c.family);
    }

    #[test]
    fn test_is_ace() {
        let mut c = Card::new(CardColor::SPADES, CardFamily::JACK);
        assert_eq!(c.is_ace(), false);

        c = Card::new(CardColor::SPADES, CardFamily::TEN);
        assert_eq!(c.is_ace(), false);

        c = Card::new(CardColor::SPADES, CardFamily::ACE);
        assert_eq!(c.is_ace(), true);
    }

    #[test]
    fn test_is_jack() {
        let mut c = Card::new(CardColor::SPADES, CardFamily::JACK);
        assert_eq!(c.is_jack(), true);

        c = Card::new(CardColor::SPADES, CardFamily::TEN);
        assert_eq!(c.is_jack(), false);

        c = Card::new(CardColor::SPADES, CardFamily::ACE);
        assert_eq!(c.is_jack(), false);
    }

    #[test]
    fn test_is_nine() {
        let mut c = Card::new(CardColor::SPADES, CardFamily::JACK);
        assert_eq!(c.is_jack(), true);

        c = Card::new(CardColor::SPADES, CardFamily::TEN);
        assert_eq!(c.is_jack(), false);

        c = Card::new(CardColor::SPADES, CardFamily::ACE);
        assert_eq!(c.is_jack(), false);
    }

    #[test]
    fn test_is_ten() {
        let mut c = Card::new(CardColor::SPADES, CardFamily::TEN);
        assert_eq!(c.is_ten(), true);

        c = Card::new(CardColor::SPADES, CardFamily::JACK);
        assert_eq!(c.is_ten(), false);

        c = Card::new(CardColor::SPADES, CardFamily::ACE);
        assert_eq!(c.is_ten(), false);
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
}

//   static AsPique: Card = Card::new(CardColor::SPADES, CardFamily::ACE);
