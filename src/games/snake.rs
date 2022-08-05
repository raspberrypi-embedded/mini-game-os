use alloc::collections::VecDeque;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use crate::board::driver::uart_read;
use crate::graphics::Graphics;
use crate::kdebug;
use crate::timer::{get_counter, wait_msec};
use alloc::format;
use rand::{ SeedableRng, RngCore};
use rand::rngs::SmallRng;
use core::usize;

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up, 
    Down
}

#[derive(PartialEq)]
pub enum GameState {
    Stop,
    Start,
    Fail
}

#[derive(PartialEq)]
pub enum Speed {
    Slow,
    Medium,
    Fast
}

/// The attribute of game board
struct GameBoard {
    pub(crate) top: u32,
    pub(crate) bottom: u32,
    pub(crate) left: u32, 
    pub(crate) right: u32
}

impl GameBoard {
    pub(crate) fn new(top: u32, bottom: u32, left: u32, right: u32) -> Self {
        Self {
            top,
            bottom,
            left, 
            right
        }
    }
}

pub struct Snake<'a> {
    score: u32,
    graphics: &'a mut Graphics,
    body: VecDeque<Point>,
    len: usize,
    rec_width: u32,
    direction: Direction,
    state: GameState,
    board: GameBoard,
    food: VecDeque<Point>
}

impl<'a> Snake<'a> {
    pub fn new(rec_width: u32, graphics: &'a mut Graphics) -> Self {
        let top = (graphics.height() / 100 ) * 10;
        let bottom = (graphics.height() / 100) * 90;
        let left = (graphics.width() / 100) * 10;
        let right = (graphics.width() / 100) * 90;
        Self {
            score: 0,
            graphics,
            body: VecDeque::new(),
            len: 0,
            rec_width,
            direction: Direction::Right,
            state: GameState::Stop,
            board: GameBoard::new(
                top,
                bottom,
                left,
                right
            ),
            food: VecDeque::new()
        }
    }

    pub fn init(&mut self) {
        self.len = 10;
        let seed = get_counter();
        let mut rand_num =  SmallRng::seed_from_u64(seed as u64);
        let x0 = (rand_num.next_u32() % (self.board.right - self.board.left)) * 3 / 5 + self.board.left;
        let y0 = (rand_num.next_u32() % (self.board.bottom - self.board.top)) * 3 / 5 + self.board.top;
        kdebug!("x0: {}, y0: {}", x0, y0);
        for i in 0..10 {
            let x = x0 + i * self.rec_width;
            let y = y0;
            self.body.push_back(Point::new(x as i32, y as i32))
        }
    }

    fn display_head(&mut self){
        let width = self.graphics.width();
        let height = self.graphics.height();
        let logo_height = (height / 100) * 5;
        let score_height = logo_height;
        let logo_width = (width / 100) * 30;
        let score_width = (width / 100) * 60;
        let score_text = format!("Score: {}", self.score);
        self.graphics.draw_text("Snake Games", logo_width, logo_height);
        self.graphics.draw_text(score_text.as_str(), score_width, score_height);
    }

    fn display_board(&mut self) {
        // draw snake in board
        for p in self.body.iter() {
            let x = p.x;
            let y = p.y;
            self.graphics.draw_rectangle(
                x, 
                y, 
                self.rec_width, 
                self.rec_width, 
                Rgb888::WHITE
            )
        }

        for f in self.food.iter() {
            let x = f.x;
            let y = f.y;
            self.graphics.draw_rectangle(
                x, 
                y, 
                self.rec_width, 
                self.rec_width, 
                Rgb888::WHITE
            )
        }
    }

    /// generate food
    fn generate_food(&mut self) {
        let seed = get_counter() as u64;
        let mut rand = SmallRng::seed_from_u64(seed);
        let x = (rand.next_u32() % (self.board.right - self.board.left)) * 3 / 5 + self.board.left;
        let y = (rand.next_u32() % (self.board.bottom - self.board.top)) * 3 / 5 + self.board.top;
        self.food.push_back(Point::new(x as i32, y as i32));
    }

    /// check if snake eat food 
    fn handle_eat(&mut self) {
        let tail = self.body.back().unwrap();
        let x = tail.x;
        let y = tail.y;
        let mut index = usize::MAX;
        for i in 0..self.food.len() {
            let f = &self.food[i];
            if (x - f.x).abs() <= self.rec_width as i32 && (y - f.y).abs() <= self.rec_width as i32 {
                // remove food block from board
                index = i;
                self.len += 1;
                self.score += 1;
                let head = self.body.front().unwrap();
                let mut point = Point::new(0, 0);
                kdebug!("x: {}, y: {}", x, y);
                match self.direction {
                    Direction::Left => {
                        point = Point::new(head.x + self.rec_width as i32, head.y);
                    }
                    Direction::Right => {
                        point = Point::new(head.x - self.rec_width as i32, head.y);
                    }
                    Direction::Down => {
                        point = Point::new(head.x, head.y - self.rec_width as i32);
                    }
                    Direction::Up => {
                        point = Point::new(head.x, head.y + self.rec_width as i32);
                    }
                }
                self.body.push_back(point);
            }
        }
        if !self.food.is_empty() && index < self.food.len() {
            self.graphics.draw_rectangle(
                self.food[index].x, 
                self.food[index].y, 
                self.rec_width, 
                self.rec_width, 
                Rgb888::BLACK
            );
            self.food.remove(index);
        }
    }

    /// display the boundary of board
    fn display_frame(&mut self) {
        self.graphics.draw_line(
            self.board.left as i32, 
            self.board.top as i32, 
            self.board.right as i32 , 
            self.board.top as i32
        );
        self.graphics.draw_line(
            self.board.left as i32, 
            self.board.bottom as i32, 
            self.board.right as i32 , 
            self.board.bottom as i32
        );

        self.graphics.draw_line(
            self.board.left as i32, 
            self.board.top as i32, 
            self.board.left as i32 , 
            self.board.bottom as i32
        );

        self.graphics.draw_line(
            self.board.right as i32, 
            self.board.top as i32, 
            self.board.right as i32 , 
            self.board.bottom as i32
        );
    }

    pub fn display(&mut self) {
        self.display_head();
        self.display_board();
        self.display_frame();
    }

    pub fn check_fail(&mut self) -> bool {
        let tail = self.body.back().unwrap();
        let x = tail.x as u32;
        let y = tail.y as u32;
        if x > self.board.right || x < self.board.left || y > self.board.bottom || y < self.board.top {
            kdebug!("right: {}, left: {}, top: {}, bottom: {}", self.board.right, self.board.left, self.board.top, self.board.bottom);
            kdebug!("x: {}, y: {}", x, y);
            return true
        }
        false
    }

    pub fn play(&mut self) {
        self.state = GameState::Start;
        let mut rounds = 0;
        while self.state == GameState::Start {
            rounds += 1;
            // check if eat food and update the state.
            self.handle_eat();
            if self.check_fail() {
                self.state = GameState::Fail;
                kdebug!("Snake Game Over!");
            }
            if let Some(c) = uart_read() {
                match c {
                    'w' => { self.direction = Direction::Up },
                    's' => { self.direction = Direction::Down },
                    'a' => { self.direction = Direction::Left },
                    'd' => { self.direction = Direction::Right },
                    _ => {
                        kdebug!("Invalid input {}!", c)
                    }
                }
            }


            match self.direction {
                Direction::Right => {
                    let tail = self.body.back().unwrap();
                    let point = Point::new(tail.x + self.rec_width as i32, tail.y);
                    self.body.push_back(point);
                }

                Direction::Left => {
                    let tail = self.body.back().unwrap();
                    let point = Point::new(tail.x - self.rec_width as i32, tail.y);
                    self.body.push_back(point);
                }

                Direction::Up => {
                    let tail = self.body.back().unwrap();
                    let point = Point::new(tail.x, tail.y - self.rec_width as i32);
                    self.body.push_back(point);
                }

                Direction::Down => {
                    let tail = self.body.back().unwrap();
                    let point = Point::new(tail.x, tail.y + self.rec_width as i32);
                    self.body.push_back(point);
                }
            }
            wait_msec(100);
            let head = self.body.pop_front().unwrap();
            // Generate food in board each 5 rounds
            if rounds % 20 == 0 {
                self.generate_food();
            }
            // remove snake tail
            self.graphics.draw_rectangle(
                head.x, 
                head.y, 
                self.rec_width, 
                self.rec_width, 
                Rgb888::BLACK
            );
            self.display();
        }
    }
}