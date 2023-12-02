use crate::cards::{Card, CardColor, CardFamily};
use crate::iplayers::IPlayer;

#[derive(Debug)]
pub struct Game {
    players: Vec<Box<dyn IPlayer>>,
    cards: Vec<Card>,
}

impl Game {
    fn shuffle_cards() -> Vec<Card> {
        let mut cards: Vec<Card> = vec![];
        let colors = vec![
            CardColor::CLUBS,
            CardColor::DIAMONDS,
            CardColor::HEARTS,
            CardColor::SPADES,
        ];
        let families = vec![
            CardFamily::ACE,
            CardFamily::SEVEN,
            CardFamily::HEIGHT,
            CardFamily::NINE,
            CardFamily::TEN,
            CardFamily::JACK,
            CardFamily::KING,
            CardFamily::QUEEN,
        ];

        for color in colors {
            for family in &families {
                cards.push(Card::new(color.clone(), family.clone()));
            }
        }
        return cards;
    }

    pub fn new() -> Self {
        Self {
            players: vec![],
            cards: Self::shuffle_cards(),
        }
    }

    fn add_player(&mut self, p: Box<dyn IPlayer>) {
        if self.players.len() < 4 {
            self.players.push(p)
        }
    }
}

#[cfg(test)]
mod game_test {
    use crate::game::Game;
    use crate::machine_player::MachinePlayer;

    #[test]
    fn test_new_game() {
        let g = Game::new();

        assert_eq!(g.players.len(), 0);
        assert_eq!(g.cards.len(), 32);
    }

    #[test]
    fn test_add_player() {
        let mut g = Game::new();
        for i in 0..4 {
          g.add_player(Box::new(MachinePlayer::new()));
          assert_eq!(g.players.len(), i+1);
        }

        g.add_player(Box::new(MachinePlayer::new()));
        assert_eq!(g.players.len(), 4);
    }

}
