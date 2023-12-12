use crate::cards::Card;
use crate::gametake::GameTake;

#[derive(Debug)]
pub enum Message {
    PleaseTake(crossbeam_channel::Sender<PlayerMessage>),
    PleasePlay(crossbeam_channel::Sender<PlayerMessage>),
    TakingCards(Vec<Card>),
    PlayingCards(Vec<Card>),
    Deck(Vec<Card>),
}

#[derive(Debug)]
pub enum PlayerMessage {
    SetPlayerTake(GameTake),
    PlayCard(Card),
}
