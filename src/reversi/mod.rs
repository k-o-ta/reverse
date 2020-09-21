use bit_vec::BitVec;
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

struct Board {
    board: [[Color; Self::SIZE + 2]; Self::SIZE + 2],
    update_log: Vec<Vec<Point>>,
    movable_dir: [[[BitVec; Self::SIZE + 2]; Self::SIZE + 2]; Self::MAX_TURNS + 1],
    movable_positions: [Vec<Point>; Self::MAX_TURNS + 1],
    turns: usize,
    discs: ColorStorage<u32>,
    current_color: Color,
}

struct ColorStorage<T>([T; 3]);
impl<T> ColorStorage<T> {
    fn get(&self, color: &Color) -> &T {
        &self.0[(color + 1) as usize]
    }
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

impl Board {
    const SIZE: usize = 8;
    const MAX_TURNS: usize = 60;
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
    fn movability(&self, disc: &Disc) -> BitVec {
        let mut direction = BitVec::from_elem(9, false);
        if self.board[disc.x()][disc.y()] == EMPTY {
            return direction;
        }

        // UPPER
        if self.board[disc.x()][disc.y() - 1] == -disc.color {
            let x = disc.x();
            let mut y = disc.y() - 2;
            while self.board[x][y] == -disc.color {
                y = y - 1;
            }
            if self.board[x][y] == disc.color {
                direction.set(Self::DIRECTION_UPPER, true);
            }
        }
        // LOWER
        if self.board[disc.x()][disc.y() + 1] == -disc.color {
            let x = disc.x();
            let mut y = disc.y() + 2;
            while self.board[x][y] == disc.color {
                y = y + 1;
            }
            if self.board[x][y] == disc.color {
                direction.set(Self::DIRECTION_LOWER, true);
            }
        }

        // LEFT
        if self.board[disc.x() - 1][disc.y()] == -disc.color {
            let mut x = disc.x() - 2;
            let y = disc.y();
            while self.board[x][y] == disc.color {
                x = x - 1;
            }
            if self.board[x][y] == disc.color {
                direction.set(Self::DIRECTION_LEFT, true);
            }
        }

        // RIGHT
        if self.board[disc.x() + 1][disc.y()] == -disc.color {
            let mut x = disc.x() + 2;
            let y = disc.y();
            while self.board[x][y] == disc.color {
                x = x + 1;
            }
            if self.board[x][y] == disc.color {
                direction.set(Self::DIRECTION_RIGHT, true);
            }
        }

        // UPPER_RIGHT
        if self.board[disc.x() + 1][disc.y() - 1] == -disc.color {
            let mut x = disc.x() + 2;
            let mut y = disc.y() - 2;
            while self.board[x][y] == disc.color {
                x = x + 1;
                y = y - 1;
            }
            if self.board[x][y] == disc.color {
                direction.set(Self::DIRECTION_UPPER_RIGHT, true);
            }
        }

        // UPPER_LEFT
        if self.board[disc.x() - 1][disc.y() - 1] == -disc.color {
            let mut x = disc.x() - 2;
            let mut y = disc.y() - 2;
            while self.board[x][y] == disc.color {
                x = x - 1;
                y = y - 1;
            }
            if self.board[x][y] == disc.color {
                direction.set(Self::DIRECTION_UPPER_LEFT, true);
            }
        }

        // LOWER_LEFT
        if self.board[disc.x() - 1][disc.y() + 1] == -disc.color {
            let mut x = disc.x() - 2;
            let mut y = disc.y() + 2;
            while self.board[x][y] == disc.color {
                x = x - 1;
                y = y + 1;
            }
            if self.board[x][y] == disc.color {
                direction.set(Self::DIRECTION_LOWER_LEFT, true);
            }
        }

        // LOWER_RIGHT
        if self.board[disc.x() + 1][disc.y() + 1] == -disc.color {
            let mut x = disc.x() + 2;
            let mut y = disc.y() + 2;
            while self.board[x][y] == disc.color {
                x = x + 1;
                y = y + 1;
            }
            if self.board[x][y] == disc.color {
                direction.set(Self::DIRECTION_LOWER_RIGHT, true);
            }
        }

        direction
    }

    fn move_disc(&mut self, point: &Point) -> bool {
        if point.x < 0 || point.x >= Self::SIZE {
            return false;
        }
        if point.y < 0 || point.y >= Self::SIZE {
            return false;
        }
        if self.movable_dir[self.turns][point.x][point.y].none() {
            return false;
        }

        self.flip_discs();

        self.turns = self.turns + 1;
        self.current_color = -self.current_color;

        self.init_movable();

        true
    }

    fn flip_discs(&mut self) {}
    fn init_movable(&mut self) {
        let mut disc = Disc::new(Point::new(0, 0), self.current_color);
        self.movable_positions[self.turns].clear();
        for x in 1..=(Self::SIZE) {
            disc.point.x = x;
            for y in 1..=(Self::SIZE) {
                disc.point.y = y;
                let direction = self.movability(&disc);
                if !direction.none() {
                    self.movable_positions[self.turns].push(disc.point.clone())
                }
                self.movable_dir[self.turns][x][y] = direction;
            }
        }
    }
}
