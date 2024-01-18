use crossterm::{cursor::MoveTo, execute};
use std::{collections::VecDeque, io};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Snake {
    deque: VecDeque<(u8, u8)>,
}

impl Snake {
    pub fn new() -> Snake {
        draw_snake_body_segment(0, 0);
        Snake {
            deque: VecDeque::from([(0, 0)]),
        }
    }

    pub fn head(&self) -> (u8, u8) {
        self.deque[0]
    }

    pub fn move_dir(&mut self, dir: Direction) {
        let position = change_position_by_direction(self.deque[0], dir);

        draw_snake_body_segment(position.0, position.1);
        let last = self.deque[self.deque.len() - 1];

        clear_pixel(last.0, last.1);

        self.deque.rotate_right(1);
        self.deque[0] = position;
    }

    pub fn expand(&mut self, dir: Direction) {
        let position = change_position_by_direction(self.deque[0], dir);

        draw_snake_body_segment(position.0, position.1);

        self.deque.push_front(position);
    }
}

fn change_position_by_direction(pos: (u8, u8), dir: Direction) -> (u8, u8) {
    let mut p = (pos.0 as i16, pos.1 as i16);

    match dir {
        Direction::Up => p.1 -= 1,
        Direction::Down => p.1 += 1,
        Direction::Left => p.0 -= 1,
        Direction::Right => p.0 += 1,
    }

    if p.0 < 0 {
        p.0 = crate::GAME_WIDTH as i16 - 1;
    }
    p.0 %= crate::GAME_WIDTH as i16;

    if p.1 < 0 {
        p.1 = crate::GAME_HEIGHT as i16 - 1;
    }
    p.1 %= crate::GAME_HEIGHT as i16;

    (p.0 as u8, p.1 as u8)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn draw_snake_body_segment(x: u8, y: u8) {
    draw_pixel(x, y, "*");
}

fn clear_pixel(x: u8, y: u8) {
    draw_pixel(x, y, " ");
}

pub fn draw_pixel(x: u8, y: u8, text: &str) {
    _ = execute!(io::stdout(), MoveTo((x) as u16, (y) as u16));
    print!("{}", text);
    _ = execute!(io::stdout(), MoveTo(0, 20));
}
