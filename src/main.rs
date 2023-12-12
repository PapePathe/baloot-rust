use core::time;
use std::thread;

use belote::game::Game;
use belote::machine_player::MachinePlayer;
use belote::message::PlayerMessage;

use crossbeam_channel::unbounded;

fn main() {
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
                println!("Game started {}", g.started);
                g.add_take(0, take.clone(), s.clone());
                println!("Takes {:?}", g.takes);
                if g.started {
                    println!("Game started {}", g.started);
                    println!("Takes {:?}", g.takes);
                    break 'taking_loop;
                }
            }
        }

        'playing_loop: loop {
            let received: &PlayerMessage = &r.recv().unwrap();

            if let PlayerMessage::PlayCard(c) = received {
                g.add_card(*c, s.clone());
                println!("Received card");
                println!("{:?}", c);
                println!(
                    "decks len: {:?} values: {:?}",
                    g.decks.len(),
                    g.decks[g.current_deck]
                );

                if g.finished {
                    break 'playing_loop;
                }
            }
        }

        thread::sleep(time::Duration::from_millis(1500));
        println!();
        println!();
        println!();
    }
}
