use bit_vec::BitVec;
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

const BOARD_SIZE: usize = 8;
struct Board([[Color; BOARD_SIZE + 2]; BOARD_SIZE + 2]);

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

// DIRECTION
// 107
// 2*6
// 345
const DIRECTION_UPPER: usize = 0;
const DIRECTION_UPPER_LEFT: usize = 1;
const DIRECTION_LEFT: usize = 2;
const DIRECTION_LOWER_LEFT: usize = 3;
const DIRECTION_LOWER: usize = 4;
const DIRECTION_LOWER_RIGHT: usize = 5;
const DIRECTION_RIGHT: usize = 6;
const DIRECTION_UPPER_RIGHT: usize = 7;
impl Board {
    fn movability(&self, disc: &Disc) -> BitVec {
        let mut direction = BitVec::from_elem(9, false);
        if self.0[disc.x()][disc.y()] == EMPTY {
            return direction;
        }

        // UPPER
        if self.0[disc.x()][disc.y() - 1] == -disc.color {
            let x = disc.x();
            let mut y = disc.y() - 2;
            while self.0[x][y] == -disc.color {
                y = y - 1;
            }
            if self.0[x][y] == disc.color {
                direction.set(DIRECTION_UPPER, true);
            }
        }
        // LOWER
        if self.0[disc.x()][disc.y() + 1] == -disc.color {
            let x = disc.x();
            let mut y = disc.y() + 2;
            while self.0[x][y] == disc.color {
                y = y + 1;
            }
            if self.0[x][y] == disc.color {
                direction.set(DIRECTION_LOWER, true);
            }
        }

        // LEFT
        if self.0[disc.x() - 1][disc.y()] == -disc.color {
            let mut x = disc.x() - 2;
            let y = disc.y();
            while self.0[x][y] == disc.color {
                x = x - 1;
            }
            if self.0[x][y] == disc.color {
                direction.set(DIRECTION_LEFT, true);
            }
        }

        // RIGHT
        if self.0[disc.x() + 1][disc.y()] == -disc.color {
            let mut x = disc.x() + 2;
            let y = disc.y();
            while self.0[x][y] == disc.color {
                x = x + 1;
            }
            if self.0[x][y] == disc.color {
                direction.set(DIRECTION_RIGHT, true);
            }
        }

        // UPPER_RIGHT
        if self.0[disc.x() + 1][disc.y() - 1] == -disc.color {
            let mut x = disc.x() + 2;
            let mut y = disc.y() - 2;
            while self.0[x][y] == disc.color {
                x = x + 1;
                y = y - 1;
            }
            if self.0[x][y] == disc.color {
                direction.set(DIRECTION_UPPER_RIGHT, true);
            }
        }

        // UPPER_LEFT
        if self.0[disc.x() - 1][disc.y() - 1] == -disc.color {
            let mut x = disc.x() - 2;
            let mut y = disc.y() - 2;
            while self.0[x][y] == disc.color {
                x = x - 1;
                y = y - 1;
            }
            if self.0[x][y] == disc.color {
                direction.set(DIRECTION_UPPER_LEFT, true);
            }
        }

        // LOWER_LEFT
        if self.0[disc.x() - 1][disc.y() + 1] == -disc.color {
            let mut x = disc.x() - 2;
            let mut y = disc.y() + 2;
            while self.0[x][y] == disc.color {
                x = x - 1;
                y = y + 1;
            }
            if self.0[x][y] == disc.color {
                direction.set(DIRECTION_LOWER_LEFT, true);
            }
        }

        // LOWER_RIGHT
        if self.0[disc.x() + 1][disc.y() + 1] == -disc.color {
            let mut x = disc.x() + 2;
            let mut y = disc.y() + 2;
            while self.0[x][y] == disc.color {
                x = x + 1;
                y = y + 1;
            }
            if self.0[x][y] == disc.color {
                direction.set(DIRECTION_LOWER_RIGHT, true);
            }
        }

        direction
    }
}
