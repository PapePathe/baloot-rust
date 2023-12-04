use belote::game::Game;
use belote::machine_player::MachinePlayer;
use belote::iplayers::IPlayer;
use belote::gametake::Take;
use belote::gametake::GameTake;

fn main() {
    let mut g: Game<MachinePlayer> = Game::new();

    g.add_player(MachinePlayer::new(vec![]));
    g.add_player(MachinePlayer::new(vec![]));
    g.add_player(MachinePlayer::new(vec![]));
    g.add_player(MachinePlayer::new(vec![]));


    for mut p in g.players {
        for c in p.get_cards()  {
           println!("card: {:?} value: {}", c, c.evaluate_for_take(GameTake::Cent(Take::CENT)));
        }
        println!("")
    }

}
