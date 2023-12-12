use crate::cards::Card;
use crate::gametake::GameTake;
use postage::dispatch::Sender;

#[derive(Debug)]
pub enum Message {
    PleaseTake,
    PleasePlay,
    TakingCards(Vec<Card>),
    PlayingCards(Vec<Card>),
    Deck(Vec<Card>),
}

#[derive(Debug)]
pub enum PlayerMessage {
    SetPlayerTake(GameTake),
    PlayCard(Card),
}
