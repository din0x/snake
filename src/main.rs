mod snake;

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::prelude::*;
use snake::{Direction, Snake};
use std::{thread, time::Duration};

const GAME_HEIGHT: u8 = 10;
const GAME_WIDTH: u8 = 2 * GAME_HEIGHT;

fn main() {
    let mut snake = Snake::new();
    let mut current_direction = Direction::Right;

    let mut food: (u8, u8) = gen_food();
    snake::draw_pixel(food.0, food.1, "o");

    loop {
        if let Some(direction) = get_direction() {
            current_direction = direction;
        }

        if snake.head() == food {
            food = gen_food();
            snake::draw_pixel(food.0, food.1, "o");
            snake.expand(current_direction);
        } else {
            snake.move_dir(current_direction);
        }

        thread::sleep(Duration::from_millis(250));
    }
}

fn event_avilable() -> bool {
    poll(Duration::ZERO).is_ok_and(|x| x)
}

fn gen_food() -> (u8, u8) {
    let mut rng = rand::thread_rng();
    (
        rng.gen_range(0..crate::GAME_WIDTH),
        rng.gen_range(0..crate::GAME_HEIGHT),
    )
}

fn get_direction() -> Option<Direction> {
    if !event_avilable() {
        return None;
    }

    let key_binds: &[(&[KeyCode], Direction)] = &[
        (&[KeyCode::Char('w')], Direction::Up),
        (&[KeyCode::Char('s')], Direction::Down),
        (&[KeyCode::Char('a')], Direction::Left),
        (&[KeyCode::Char('d')], Direction::Right),
    ];

    let mut direction: Option<Direction> = None;

    match event::read() {
        Ok(Event::Key(key_event)) => {
            for bind in key_binds {
                match key_event {
                    KeyEvent {
                        code,
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        state: _,
                    } => {
                        for key in bind.0 {
                            if direction.is_none() && *key == code {
                                direction = Some(bind.1);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    };

    direction
}
