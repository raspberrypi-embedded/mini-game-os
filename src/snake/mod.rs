use alloc::collections::VecDeque; 
use embedded_graphics::geometry::Point;
use crate::graphics::Graphics;

pub enum Direction {
    Left,
    Right,
    Up, 
    Down
}
pub struct Snake<'a> {
    body: VecDeque<Point>,
    len: usize,
    width: usize,
    direction: Direction,
    graphics: &'a mut Graphics
}

impl<'a> Snake<'a> {
    pub fn new(width: usize, graphics: &'a mut Graphics) -> Self {
        Self {
            body: VecDeque::new(),
            len: 0,
            width,
            direction: Direction::Right,
            graphics
        }
    }

    pub fn init(&mut self) {
        self.len = 100;
        for i in 50..(self.len + 50) {
            let point = Point::new(i as i32, i as i32);
            self.body.push_back(point);
        }
    }

    pub fn display(&mut self) {
        for point in self.body.iter() {
            self.graphics.draw_line(point.x, point.y, point.x, point.y + (self.width as i32))
        }
    }
}