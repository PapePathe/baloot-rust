use crate::cards::Card;

pub enum Message {
    PleaseTake,
    PleasePlay,
    TakingCards(Vec<Card>),
    PlayingCards(Vec<Card>),
    Deck(Vec<Card>),
}
