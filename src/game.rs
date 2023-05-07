use std::io::Stdout;

use crossterm::{ExecutableCommand, cursor};
use rand::Rng;

use crate::GameInput;

#[derive(Clone, Copy)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

pub struct Snake {
    pos: Coordinate,
    pub pos_history: Vec<Coordinate>,
    pub size: usize
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            pos: Coordinate { x: 1, y: 1 },
            pos_history: Vec::new(),
            size: 3,
        }
    }

    pub fn update(&mut self, width: usize, height: usize, direction: GameInput) {
        self.pos_history.insert(0, self.pos);
        if self.pos_history.len() >= self.size {
            self.pos_history.drain(self.size..);
        }

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
    pub food_pos: Option<Coordinate>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        Map {
            map_width: width,
            map_height: height,
            map: vec![vec![0; width]; height],
            food_pos: None,
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

    pub fn spawn_food(&mut self) {
        let mut food_x = rand::thread_rng().gen_range(0..self.map_width);
        let mut food_y = rand::thread_rng().gen_range(0..self.map_height);
        while self.map[food_y][food_x] != 0 {
            food_x = rand::thread_rng().gen_range(0..self.map_width);
            food_y = rand::thread_rng().gen_range(0..self.map_height);
        }

        self.food_pos = Some( Coordinate { x: food_x, y: food_y } )
    }

    pub fn update(&mut self, snake: &Snake) {
        self.map = vec![vec![0; self.map_width]; self.map_height];

        self.replace_element(snake.pos, 1);
        for i in &snake.pos_history {
            self.replace_element(*i, 1);
        }

        self.spawn_food();
        if self.food_pos.is_some() {
            self.replace_element(self.food_pos.unwrap(), 2)
        }
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
