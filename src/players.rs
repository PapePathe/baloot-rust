use crate::cards::Card;
use crate::message::Message;
use crate::iplayers::IPlayer;

struct Player {
    name: String,
    cards: Vec<Card>,
}

impl Player {
    fn new(name: String, cards: Vec<Card>) -> Self {
        Self {
            name: name,
            cards: cards,
        }
    }
}


impl IPlayer for Player {
    fn send_message(&mut self, _m: Message) {}
}

#[cfg(test)]
mod player_test {
    use crate::players::*;

    #[test]
    fn test_new_player() {
        let p = Player::new(String::from("pathe"), vec![]);
        assert_eq!(p.name, "pathe");
        assert_eq!(p.cards.len(), 0);
    }
}
