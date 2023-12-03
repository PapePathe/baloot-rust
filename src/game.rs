use crate::cards::{Card, CardColor, CardFamily};
use crate::iplayers::IPlayer;
use crate::gametake::GameTake;
use crate::gametake::Take;
use crate::message::Message;
use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Game<T: IPlayer> {
    pub players: Vec<T>,
    cards: Vec<Card>,
    take: GameTake
}

impl<T> Default for Game<T>
where
    T: IPlayer,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Game<T>
where
    T: IPlayer,
{
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
                cards.push(Card::new(color, *family));
            }
        }
        cards.shuffle(&mut thread_rng());

        cards
    }

    pub fn new() -> Self {
        Self {
            players: vec![],
            cards: Self::shuffle_cards(),
            take: GameTake::Skip(Take::SKIP),
        }
    }

    pub fn add_player(&mut self, mut player: T) {
        if self.players.len() < 4 {
            let mut pcards: Vec<Card> = vec![];

            for i in 0..5 {
                let c = self.cards[i];
                pcards.push(c);
                if let Some(pos) = self.cards.iter().position(|x| *x == c) {
                    self.cards.remove(pos);
                }
            }
            player.send_message(Message::TakingCards(pcards));
            self.players.push(player);
        }
    }

    pub fn add_take(&mut self, mut player: T, take: GameTake) {
        self.take = take.clone();

        match take {
            GameTake::Tout(_) => {
                for mut p in &mut self.players  {
                    let mut pcards = p.get_cards();

                    for i in 0..3 {
                        let c = self.cards[i];
                        pcards.push(c);
                        if let Some(pos) = self.cards.iter().position(|x| *x == c) {
                            self.cards.remove(pos);
                        }
                    }
                    p.send_message(Message::PlayingCards(pcards))

                    //p.set_cards(pcards);

                }
            }
            _ => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod game_test {
    use crate::game::Game;
    use crate::machine_player::MachinePlayer;
    use crate::players::Player;
    use crate::gametake::GameTake;
    use crate::gametake::Take;
    use crate::iplayers::IPlayer;

    #[test]
    fn test_new_game_with_machine_players() {
        let g: Game<MachinePlayer> = Game::new();

        assert_eq!(g.players.len(), 0);
        assert_eq!(g.cards.len(), 32);
    }


    #[test]
    fn test_new_game_with_human_players() {
        let g: Game<Player> = Game::new();

        assert_eq!(g.players.len(), 0);
        assert_eq!(g.cards.len(), 32);
    }

    #[test]
    fn test_add_take_that_starts_the_game_by_machine_player() {
        let mut g: Game<MachinePlayer> = Game::new();
        let p = MachinePlayer::new(vec![]);
        g.add_player(p.clone());
        g.add_take(p, GameTake::Tout(Take::TOUT));

        assert_eq!(g.take, GameTake::Tout(Take::TOUT));

        for mut p in g.players  {
            assert_eq!(p.get_cards().len(), 5);
            assert_eq!(p.get_playing_cards().len(), 8);
        }
    }


    #[test]
    fn test_add_take_that_starts_the_game_by_human_player() {
        let mut g: Game<Player> = Game::new();
        let p = Player::new(String::from("pathe"), vec![]);
        g.add_player(p.clone());
        g.add_take(p, GameTake::Tout(Take::TOUT));

        assert_eq!(g.take, GameTake::Tout(Take::TOUT));

        for mut p in g.players  {
            assert_eq!(p.get_cards().len(), 5);
            assert_eq!(p.get_playing_cards().len(), 8);
        }
    }

    #[test]
    fn test_add_player() {
        let mut g = Game::new();
        for i in 0..4 {
            let p = MachinePlayer::new(vec![]);
            g.add_player(p.clone());

            assert_eq!(g.players.len(), i + 1);
            assert_eq!(g.players[i].get_cards().len(), 5);
            assert_eq!(g.cards.len(), 32 - (i + 1) * 5);
        }

        g.add_player(MachinePlayer::new(vec![]));
        assert_eq!(g.players.len(), 4);
    }
}
