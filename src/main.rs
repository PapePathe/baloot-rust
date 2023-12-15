use core::time;
use std::thread;

use belote::game::Game;
use belote::machine_player::MachinePlayer;
use belote::message::PlayerMessage;

use crossbeam_channel::unbounded;
use paris::Logger;

fn main() {
    let mut log = Logger::new();

    loop {
        let (s, r) = unbounded::<PlayerMessage>();

        let mut g: Game = Game::new();

        g.add_player(Box::new(MachinePlayer::new(vec![])), s.clone());
        g.add_player(Box::new(MachinePlayer::new(vec![])), s.clone());
        g.add_player(Box::new(MachinePlayer::new(vec![])), s.clone());
        g.add_player(Box::new(MachinePlayer::new(vec![])), s.clone());

        'taking_loop: loop {
            let received: &PlayerMessage = &r.recv().unwrap();

            if let PlayerMessage::SetPlayerTake(take) = received {
                g.add_take(0, take.clone(), s.clone());
                log.indent(1)
                    .info("Take constraints")
                    .indent(2)
                    .success(format!("{:?}", g.takes));

                if g.started {
                    log.indent(1)
                        .info(format!("Takes {:?}", g.takes))
                        .indent(2)
                        .info(format!("Starting gameplay with {:?}", g.take));
                    break 'taking_loop;
                }
            }
        }

        'playing_loop: loop {
            let received: &PlayerMessage = &r.recv().unwrap();

            if let PlayerMessage::PlayCard(c) = received {
                g.add_card(*c, s.clone());
                log.info("Received card")
                    .indent(1)
                    .success(format!("{:?}", c))
                    .indent(2)
                    .warn(format!("current deck {:?}", g.decks[g.current_deck]));

                if g.finished {
                    break 'playing_loop;
                }
            }
        }

        thread::sleep(time::Duration::from_millis(15000));
        println!();
        println!();
        println!();
    }
}
