use crate::message::Message;
use crate::cards::Card;
use crate::iplayers::IPlayer;

struct MachinePlayer {
    cards: Vec<Card>,
    playing_cards: Vec<Card>,
}

impl IPlayer for MachinePlayer {
    fn send_message(&mut self, m: Message) {
        match m {
            Message::TakingCards(cards) => {
                self.cards = cards;
            }
            Message::PlayingCards(cards) => {
                self.playing_cards = cards;
            }
            _ => {
                println!("Was no able to match message")
            }
        }
    }
}

#[cfg(test)]
mod machine_player_test {
    use crate::cards::*;
    use crate::message::Message;
    use crate::machine_player::MachinePlayer;
    use crate::iplayers::IPlayer;

    #[test]
    fn test_send_message_for_deck() {
    }

    #[test]
    fn test_send_message_for_playing_cards() {
        let mut p = MachinePlayer { cards: vec![] ,playing_cards: vec![] };

        let hand = vec![Card::new(CardColor::SPADES, CardFamily::ACE)];
        p.send_message(Message::PlayingCards(hand));

        assert_eq!(p.playing_cards, vec![Card::new(CardColor::SPADES, CardFamily::ACE)]);
        assert_eq!(p.cards, vec![]);
    }

    #[test]
    fn test_send_message_for_taking_cards() {
        let mut p = MachinePlayer { cards: vec![],playing_cards: vec![] };

        let hand = vec![Card::new(CardColor::SPADES, CardFamily::ACE)];
        p.send_message(Message::TakingCards(hand));

        assert_eq!(p.cards, vec![Card::new(CardColor::SPADES, CardFamily::ACE)]);
        assert_eq!(p.playing_cards, vec![]);
    }
}
