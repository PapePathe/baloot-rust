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
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[cfg(test)]
mod cards_test {
    use crate::cards::*;

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
