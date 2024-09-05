use std::{
    io,
    thread,
    sync::{mpsc, mpsc::Receiver}
};
use termion::{input::TermRead, event::Key};
use crate::snake::SnakeDirection;

pub fn spawn_input_channel() -> Receiver<SnakeDirection> {
    let (tx, rx) = mpsc::channel::<SnakeDirection>();
    const ERROR_MESSAGE: &str = "Failed to send key";

    thread::spawn(move || {
        for key in io::stdin().keys() {
            match key {
                Ok(Key::Up) => tx.send(SnakeDirection::Up).expect(ERROR_MESSAGE),
                Ok(Key::Down) => tx.send(SnakeDirection::Down).expect(ERROR_MESSAGE),
                Ok(Key::Left) => tx.send(SnakeDirection::Left).expect(ERROR_MESSAGE),
                Ok(Key::Right) => tx.send(SnakeDirection::Right).expect(ERROR_MESSAGE),
                _ => {}
            }
        }
    });

    rx
}