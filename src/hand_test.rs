use crate::cards::Card;
use crate::cards::CardColor;
use crate::cards::CardFamily;

pub fn thirty_four_color(color: CardColor) -> Vec<Card> {
    return vec![
        Card::new(color, CardFamily::JACK),
        Card::new(color, CardFamily::NINE),
        Card::new(CardColor::CLUBS, CardFamily::TEN),
        Card::new(CardColor::CLUBS, CardFamily::QUEEN),
        Card::new(CardColor::CLUBS, CardFamily::KING),
    ];
}

pub fn bad_hand_color(color: CardColor) -> Vec<Card> {
    vec![
        Card::new(color, CardFamily::SEVEN),
        Card::new(color, CardFamily::NINE),
        Card::new(CardColor::CLUBS, CardFamily::TEN),
        Card::new(CardColor::CLUBS, CardFamily::QUEEN),
        Card::new(CardColor::CLUBS, CardFamily::KING),
    ]
}
