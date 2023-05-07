use std::io::Stdout;

use crossterm::{ExecutableCommand, cursor};

use crate::GameInput;

#[derive(Clone, Copy)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

pub struct Snake {
    pos: Coordinate
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            pos: Coordinate { x: 1, y: 1 }
        }
    }

    pub fn update(&mut self, width: usize, height: usize, direction: GameInput) {
        match direction {
            GameInput::Up => {
                if self.pos.y > 0 {
                    self.pos.y = self.pos.y - 1;
                }
            },
            GameInput::Down => {
                if self.pos.y < height {
                    self.pos.y = self.pos.y + 1;
                }
            },
            GameInput::Left => {
                if self.pos.x > 0 {
                    self.pos.x = self.pos.x - 1;
                }
            },
            GameInput::Right => {
                if self.pos.x < width {
                    self.pos.x = self.pos.x + 1;
                }
            },
            GameInput::Exit => {

            },
        }
    }
}

pub struct Map {
    pub map_width: usize,
    pub map_height: usize,
    pub map: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        Map {
            map_width: width,
            map_height: height,
            map: vec![vec![0; width]; height],
        }
    }

    pub fn print(&self, stdout: &mut Stdout) {
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                print!("{} ", self.map[y][x]);
            }
            println!("");
            stdout.execute(cursor::MoveLeft((self.map_width as u16) * 2)).unwrap();
        }
    }

    pub fn replace_element(&mut self, pos: Coordinate, val: u8) {
        self.map[pos.y].push(val);
        self.map[pos.y].swap_remove(pos.x);
    }

    pub fn update(&mut self, snake: &Snake) {
        self.map = vec![vec![0; self.map_width]; self.map_height];

        self.replace_element(snake.pos, 1);
    }
}


pub struct Game {
    pub snake: Snake,
    pub map: Map,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            snake: Snake::new(),
            map: Map::new(width, height),
            width,
            height,
        }
    }

    pub fn update(&mut self, input: GameInput) {
        self.snake.update(self.width, self.height, input);
        self.map.update(&self.snake);
    }

}
