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
    update_log: Vec<Update>,
    movable_dir: [[[BitVec; Self::SIZE + 2]; Self::SIZE + 2]; Self::MAX_TURNS + 1],
    movable_positions: [Vec<Point>; Self::MAX_TURNS + 1],
    turns: usize,
    discs: ColorStorage<u32>,
    current_color: Color,
}
type Update = Vec<Disc>;

struct ColorStorage<T>([T; 3]);
impl<T> ColorStorage<T> {
    fn get(&self, color: &Color) -> &T {
        &self.0[(*color + 1) as usize]
    }
    fn set(&mut self, color: &Color, count: T) {
        self.0[(color + 1) as usize] = count;
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

        self.flip_discs(point);

        self.turns = self.turns + 1;
        self.current_color = -self.current_color;

        self.init_movable();

        true
    }

    fn flip_discs(&mut self, point: &Point) {
        let direction = &self.movable_dir[self.turns][point.x][point.y];

        self.board[point.x][point.y] = self.current_color;
        let mut update: Update = Vec::new();
        update.push(Disc::new(Point::new(point.x, point.y), self.current_color));

        if direction[Self::DIRECTION_UPPER] {
            let mut y = point.y;
            while self.board[point.x][y - 1] != self.current_color {
                y = y - 1;
                self.board[point.x][y] = self.current_color;
                update.push(Disc::new(Point::new(point.x, y), self.current_color));
            }
        }

        if direction[Self::DIRECTION_LOWER] {
            let mut y = point.y;
            while self.board[point.x][y + 1] != self.current_color {
                y = y + 1;
                self.board[point.x][y] = self.current_color;
                update.push(Disc::new(Point::new(point.x, y), self.current_color));
            }
        }

        if direction[Self::DIRECTION_LEFT] {
            let mut x = point.x;
            while self.board[x - 1][point.y] != self.current_color {
                x = x - 1;
                self.board[x][point.y] = self.current_color;
                update.push(Disc::new(Point::new(x, point.y), self.current_color));
            }
        }

        if direction[Self::DIRECTION_RIGHT] {
            let mut x = point.x;
            while self.board[x + 1][point.y] != self.current_color {
                x = x + 1;
                self.board[x][point.y] = self.current_color;
                update.push(Disc::new(Point::new(x, point.y), self.current_color));
            }
        }

        if direction[Self::DIRECTION_UPPER_RIGHT] {
            let mut x = point.x;
            let mut y = point.y;
            while self.board[x + 1][y - 1] != self.current_color {
                x = x + 1;
                y = y - 1;
                self.board[x][y] = self.current_color;
                update.push(Disc::new(Point::new(x, y), self.current_color));
            }
        }

        if direction[Self::DIRECTION_UPPER_LEFT] {
            let mut x = point.x;
            let mut y = point.y;
            while self.board[x - 1][y - 1] != self.current_color {
                x = x - 1;
                y = y - 1;
                self.board[x][y] = self.current_color;
                update.push(Disc::new(Point::new(x, y), self.current_color));
            }
        }

        if direction[Self::DIRECTION_LOWER_LEFT] {
            let mut x = point.x;
            let mut y = point.y;
            while self.board[x - 1][y + 1] != self.current_color {
                x = x - 1;
                y = y + 1;
                self.board[x][y] = self.current_color;
                update.push(Disc::new(Point::new(x, y), self.current_color));
            }
        }

        if direction[Self::DIRECTION_LOWER_RIGHT] {
            let mut x = point.x;
            let mut y = point.y;
            while self.board[x + 1][y + 1] != self.current_color {
                x = x + 1;
                y = y + 1;
                self.board[x][y] = self.current_color;
                update.push(Disc::new(Point::new(x, y), self.current_color));
            }
        }

        let disc_diff = update.len();
        self.discs.set(
            &self.current_color,
            *self.discs.get(&self.current_color) + disc_diff as u32,
        );
        self.discs.set(
            &-self.current_color,
            *self.discs.get(&-self.current_color) - (disc_diff - 1) as u32,
        );
        self.discs.set(&EMPTY, *self.discs.get(&EMPTY) - 1);
        self.update_log.push(update);
    }
    fn init_movable(&mut self) {
        self.movable_positions[self.turns].clear();
        for x in 1..=(Self::SIZE) {
            for y in 1..=(Self::SIZE) {
                let disc = Disc::new(Point::new(x, y), self.current_color);
                let direction = self.movability(&disc);
                if !direction.none() {
                    self.movable_positions[self.turns].push(disc.point)
                }
                self.movable_dir[self.turns][x][y] = direction;
            }
        }
    }
    fn is_game_over(&self) -> bool {
        if self.turns == Self::MAX_TURNS {
            return true;
        }
        if self.movable_positions[self.turns].len() != 0 {
            return false;
        }

        let mut dis = Disc::new(Point::new(0, 0), -self.current_color);
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                if !self
                    .movability(&Disc::new(Point::new(x, y), -self.current_color))
                    .none()
                {
                    return false;
                }
            }
        }
        true
    }

    fn pass(&mut self) -> bool {
        if self.movable_positions[self.turns].len() != 0 {
            return false;
        }
        if self.is_game_over() {
            return false;
        }
        self.current_color = -self.current_color;
        self.update_log.push(Vec::new());
        self.init_movable();
        true
    }
}
