use crate::game::Game;
use crate::machine_player::MachinePlayer;
use crate::players::Player;
use crate::gametake::GameTake;
use crate::gametake::Take;
use crate::iplayers::IPlayer;

#[test]
fn test_new_game_with_machine_players() {
    let g: Game = Game::new();

    assert_eq!(g.players.len(), 0);
    assert_eq!(g.cards.len(), 32);
}

#[test]
fn test_default_game_ring() {
    let g: Game = Game::new();
    assert_eq!(g.ring, vec![0, 1, 2, 3]);
}

#[test]
fn test_new_game_with_human_players() {
    let g: Game = Game::new();

    assert_eq!(g.players.len(), 0);
    assert_eq!(g.cards.len(), 32);
}

#[test]
fn test_add_take_that_starts_the_game_by_machine_player() {
    let mut g: Game = Game::new();
    let p = Box::new(MachinePlayer::new(vec![]));
    g.add_player(p.clone());
    g.add_player(p.clone());
    g.add_player(p.clone());
    g.add_player(p.clone());

    g.add_take(0, GameTake::Tout(Take::TOUT));
    assert_eq!(g.take, GameTake::Tout(Take::TOUT));
    assert_eq!(g.takes.len(), 1);

    for mut p in g.players  {
        assert_eq!(p.get_cards().len(), 5);
        assert_eq!(p.get_playing_cards().len(), 8);
    }
}

#[test]
fn test_add_four_takes_to_start_the_game_with_machine_player() {
    let mut g: Game = Game::new();
    let p1 =Box::new(MachinePlayer::new(vec![]));
    let p2 =Box::new(MachinePlayer::new(vec![]));
    let p3 =Box::new(MachinePlayer::new(vec![]));
    let p4 =Box::new(MachinePlayer::new(vec![]));

    g.add_player(p1);
    g.add_player(p2);
    g.add_player(p3);
    g.add_player(p4);

    g.add_take(0, GameTake::Club(Take::CLUBS));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 1);

    g.add_take(1, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 2);

    g.add_take(2, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 3);

    g.add_take(3, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 4);

    for mut p in g.players  {
        assert_eq!(p.get_cards().len(), 5);
        assert_eq!(p.get_playing_cards().len(), 8);
    }
}

#[test]
fn test_add_four_takes_to_start_the_game() {
    let mut g: Game = Game::new();
    let p1 =Box::new(MachinePlayer::new(vec![]));
    let p2 =Box::new(MachinePlayer::new(vec![]));
    let p3 =Box::new(MachinePlayer::new(vec![]));
    let p4 =Box::new(MachinePlayer::new(vec![]));

    g.add_player(p1);
    g.add_player(p2);
    g.add_player(p3);
    g.add_player(p4);

    g.add_take(0, GameTake::Club(Take::CLUBS));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 1);

    g.add_take(1, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 2);

    g.add_take(2, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 3);

    g.add_take(3, GameTake::Skip(Take::SKIP));
    assert_eq!(g.take, GameTake::Club(Take::CLUBS));
    assert_eq!(g.takes.len(), 4);

    for mut p in g.players  {
        assert_eq!(p.get_cards().len(), 5);
        assert_eq!(p.get_playing_cards().len(), 8);
    }
}

#[test]
fn test_add_take_that_starts_the_game_by_human_player() {
    let mut g: Game = Game::new();
    let p = Box::new(MachinePlayer::new(vec![]));
    g.add_player(p);
    g.add_take(0, GameTake::Tout(Take::TOUT));

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
        g.add_player(Box::new(p));

        assert_eq!(g.players.len(), i + 1);
        assert_eq!(g.players[i].get_cards().len(), 5);
        assert_eq!(g.cards.len(), 32 - (i + 1) * 5);
    }

    g.add_player(Box::new(MachinePlayer::new(vec![])));
    assert_eq!(g.players.len(), 4);
}
