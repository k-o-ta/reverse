pub mod board;

use std::fmt;

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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}{}", self.x - 1, self.y)
        let x = match self.x {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => panic!("unreacheble x"),
        };
        write!(f, "{}{}", x, self.y)
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
    fn from(str: &String) -> Result<Point, String> {
        let mut chars = str.chars();
        let x = match chars.next() {
            Some('a') => 1,
            Some('b') => 2,
            Some('c') => 3,
            Some('d') => 4,
            Some('e') => 5,
            Some('f') => 6,
            Some('g') => 7,
            Some('h') => 8,
            _ => return Err(String::from("x must be in a to h")),
        };
        let y = match chars.next() {
            Some(y @ '1'..='8') => y.to_digit(10).ok_or("y must be in 1 to 8")?,
            _ => return Err(String::from("y must be in 1 to 8")),
        };
        Ok(Point::new(x, y as usize))
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
