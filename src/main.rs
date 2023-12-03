use belote::game::Game;
use belote::players::Player;
use belote::machine_player::MachinePlayer;

fn main() {
    let mut g: Game<MachinePlayer> = Game::new();

    g.add_player(MachinePlayer::new(vec![]));
    g.add_player(MachinePlayer::new(vec![]));
    g.add_player(MachinePlayer::new(vec![]));
    g.add_player(MachinePlayer::new(vec![]));


    for p in g.players {
        println!("{:?}", p);
        println!("{:?}", "\n");
        println!("{:?}", "\n");
    }

}
