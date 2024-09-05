use std::{
    io,
    io::Write,
    thread,
    time,
};
use rand::Rng;
use crate::{point::Point, snake::SnakeDirection};
use crate::snake::Snake;
use crate::keyboard::spawn_input_channel;
use termion::{clear, cursor, raw::IntoRawMode};

enum GameState {
    Started,
    Over,
}

pub struct Game {
    score: usize,
    state: GameState,
    snake: Snake,
    apple: Point,
}

const GRID_SIZE: usize = 16;
const GRID_X_OFFSET: usize = 2;
const GRID_Y_OFFSET: usize = 3;
const GAME_SPEED: time::Duration = time::Duration::from_millis(250);

impl Game {
    pub fn new() -> Self {
        return Self {
            score: 0,
            state: GameState::Started,
            snake: Snake::new(),
            apple: Point::new(7, 8),
        };
    }

    pub fn start() {
        let mut game = Self::new();
        let mut last_direction: Option<SnakeDirection> = None;
        let input_channel = spawn_input_channel();

        loop {
            game.draw_game();

            while let Ok(direction) = input_channel.try_recv() {
                last_direction = Some(direction);
            }

            if let Some(direction) = last_direction.take() {
                game.snake.change_direction(direction);
            }

            if game.snake.advance().is_err() {
                game.state = GameState::Over;
                break;
            }

            let snake_head = game.snake.get_head_coordinates().expect("Error getting snake head");

            if snake_head.x == game.apple.x && snake_head.y == game.apple.y {
                let snake_tail = game.snake.get_tail_coordinates().expect("Error getting snake tail");

                game.score += 1;
                game.apple = game.get_new_apple_coordinates();

                match game.snake.direction {
                    SnakeDirection::Up => game.snake.body.insert(0, Point::new(snake_tail.x, snake_tail.y + 1)),
                    SnakeDirection::Down => game.snake.body.insert(0, Point::new(snake_tail.x, snake_tail.y - 1)),
                    SnakeDirection::Left => game.snake.body.insert(0, Point::new(snake_tail.x + 1, snake_tail.y)),
                    SnakeDirection::Right => game.snake.body.insert(0, Point::new(snake_tail.x - 1, snake_tail.y)),
                }
            } else if game.snake.ate_itself() || snake_head.x == GRID_SIZE || snake_head.y == GRID_SIZE {
                game.state = GameState::Over;
                break;
            }

            game.draw_game();
            thread::sleep(GAME_SPEED);
        }

        write!(
            io::stdout().into_raw_mode().expect("Error switching stdout to raw mode"),
            "{}",
            cursor::Show).expect("Error showing cursor"
        );
    }

    fn get_new_apple_coordinates(&self) -> Point {
        let mut rng = rand::thread_rng();
        let result = Point::new(rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));

        if !self.snake.body.contains(&result) {
            return result;
        }

        return self.get_new_apple_coordinates();
    }

    fn draw_game(&self) {
        let mut stdout = io::stdout().into_raw_mode().expect("Error switching stdout to raw mode");

        write!(stdout, "{}", clear::All).expect("Error clearing screen");
        write!(stdout, "{}", cursor::Hide).expect("Error hiding cursor");

        write!(stdout, "{}Score: {}", cursor::Goto(1, 1), self.score).expect("Error writing to stdout");
        write!(stdout, "{}{}", cursor::Goto(1, 2), "*".repeat(GRID_SIZE + GRID_X_OFFSET)).expect("Error writing board to stdout");
        write!(stdout, "{}{}", cursor::Goto(1, (GRID_SIZE + GRID_Y_OFFSET) as u16), "*".repeat(GRID_SIZE + GRID_X_OFFSET)).expect("Error writing board to stdout");

        for i in GRID_X_OFFSET..(GRID_SIZE + (GRID_X_OFFSET * 2)) {
            write!(stdout, "{}{}", cursor::Goto(1, i as u16), "*").expect("Error writing board to stdout");
            write!(stdout, "{}{}", cursor::Goto((2 + GRID_SIZE) as u16, i as u16), "*").expect("Error writing board to stdout");
        }

        self.snake.body.iter().for_each(|point| {
            write!(stdout, "{}X", cursor::Goto((point.x + GRID_X_OFFSET) as u16, (point.y + GRID_Y_OFFSET) as u16)).expect("Error writing snake to stdout");
        });

        write!(stdout, "{}O", cursor::Goto((self.apple.x + GRID_X_OFFSET) as u16, (self.apple.y + GRID_Y_OFFSET) as u16)).expect("Error writing apple to stdout");

        stdout.flush().expect("Error flushing stdout");
    }
}
