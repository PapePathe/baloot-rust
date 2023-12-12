use core::time;
use std::collections::HashMap;
use std::thread;

use belote::machine_player::MachinePlayer;
use belote::message::PlayerMessage;
use belote::players::Player;
use belote::{game::Game, message::Message};

use crossbeam_channel::{unbounded, Sender};

fn main() {
    loop {
        let (s, r) = unbounded::<PlayerMessage>();

        let mut g: Game = Game::new();

        g.add_player(Box::new(MachinePlayer::new(vec![], s.clone())));
        g.add_player(Box::new(MachinePlayer::new(vec![], s.clone())));
        g.add_player(Box::new(MachinePlayer::new(vec![], s.clone())));
        g.add_player(Box::new(MachinePlayer::new(vec![], s.clone())));

        'taking_loop: loop {
            let received: &PlayerMessage = &r.recv().unwrap();

            match received {
                PlayerMessage::SetPlayerTake(take) => {
                    println!("Game started {}", g.started);
                    g.add_take(0, take.clone());
                    println!("Takes {:?}", g.takes);
                    if g.started {
                        println!("Game started {}", g.started);
                        println!("Takes {:?}", g.takes);
                        break 'taking_loop;
                    } else {
                    }
                }
                _ => {}
            }
        }

        'playing_loop: loop {
            let received: &PlayerMessage = &r.recv().unwrap();

            match received {
                PlayerMessage::PlayCard(c) => {
                    g.add_card(c.clone());
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
                _ => {}
            }
        }

        thread::sleep(time::Duration::from_millis(1500));
        println!("");
        println!("");
        println!("");
    }
}
