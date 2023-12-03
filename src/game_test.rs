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
    g.add_player(p.clone());
    g.add_player(p.clone());
    g.add_player(p.clone());

    g.add_take(p, GameTake::Tout(Take::TOUT));
    assert_eq!(g.take, GameTake::Tout(Take::TOUT));
    assert_eq!(g.takes.len(), 1);

    for mut p in g.players  {
        assert_eq!(p.get_cards().len(), 5);
        assert_eq!(p.get_playing_cards().len(), 8);
    }
}

#[test]
fn test_add_four_takes_to_start_the_game_with_machine_player() {
    let mut g: Game<MachinePlayer> = Game::new();
    let p1 = MachinePlayer::new(vec![]);
    let p2 = MachinePlayer::new(vec![]);
    let p3 = MachinePlayer::new(vec![]);
    let p4 = MachinePlayer::new(vec![]);

    g.add_player(p1.clone());
    g.add_player(p2.clone());
    g.add_player(p3.clone());
    g.add_player(p4.clone());

    g.add_take(p1, GameTake::Club(Take::CLUBS));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 1);

    g.add_take(p2, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 2);

    g.add_take(p3, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 3);

    g.add_take(p4, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 4);

    for mut p in g.players  {
        assert_eq!(p.get_cards().len(), 5);
        assert_eq!(p.get_playing_cards().len(), 8);
    }
}

#[test]
fn test_add_four_takes_to_start_the_game() {
    let mut g: Game<Player> = Game::new();
    let p1 = Player::new(String::from("pathe"), vec![]);
    let p2 = Player::new(String::from("pathe jr"), vec![]);
    let p3 = Player::new(String::from("pathe jr jr"), vec![]);
    let p4 = Player::new(String::from("pathe jr jr jr"), vec![]);

    g.add_player(p1.clone());
    g.add_player(p2.clone());
    g.add_player(p3.clone());
    g.add_player(p4.clone());

    g.add_take(p1, GameTake::Club(Take::CLUBS));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 1);

    g.add_take(p2, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 2);

    g.add_take(p3, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 3);

    g.add_take(p4, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 4);

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
    assert_eq!(g.takes.len(), 1);

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
