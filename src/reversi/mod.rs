pub mod board;

#[derive(Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

pub type Color = i32;
const EMPTY: Color = 0;
const WHITE: Color = -1;
const BLACK: Color = 1;
const WALL: Color = 2;

pub struct Disc {
    color: Color,
    point: Point,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl Disc {
    fn new(point: Point, color: Color) -> Disc {
        Disc { point, color }
    }
    fn x(&self) -> usize {
        self.point.x
    }
    fn y(&self) -> usize {
        self.point.y
    }
}
