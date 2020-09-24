use crate::reversi::{Color, Disc, Point, BLACK, EMPTY, WALL, WHITE};
use bit_vec::BitVec;
use std::mem::{self, MaybeUninit};

pub struct Board {
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

    pub fn new() -> Board {
        // board
        let mut board: [[Color; Self::SIZE + 2]; Self::SIZE + 2] =
            [[EMPTY; Self::SIZE + 2]; Self::SIZE + 2];
        for y in 0..(Self::SIZE + 2) {
            board[0][y] = WALL;
            board[Self::SIZE + 1][y] = WALL;
        }
        for x in 0..(Self::SIZE + 2) {
            board[x][0] = WALL;
            board[x][Self::SIZE + 1] = WALL;
        }
        board[4][4] = WHITE;
        board[5][5] = WHITE;
        board[4][5] = BLACK;
        board[5][4] = BLACK;

        // discs
        let colors: [u32; 3] = [0; 3];
        let mut discs: ColorStorage<u32> = ColorStorage(colors);
        discs.set(&BLACK, 2);
        discs.set(&WHITE, 2);
        discs.set(&EMPTY, (Self::SIZE * Self::SIZE) as u32 - 4);

        let mut board = Board {
            board,
            discs,
            turns: 0,
            current_color: BLACK,
            update_log: Vec::new(),
            movable_positions: unsafe { MaybeUninit::uninit().assume_init() },
            movable_dir: unsafe { MaybeUninit::uninit().assume_init() },
            // movable_dir: [[[BitVec::from_elem(9, false); Self::SIZE + 2]; Self::SIZE + 2];
            //     Self::MAX_TURNS + 1],
            // movable_dir: [[[BitVec; Self::SIZE + 2]; Self::SIZE + 2]; Self::MAX_TURNS + 1],
        };
        board.init_movable();
        board
    }

    fn movability(&self, disc: &Disc) -> BitVec {
        let mut direction = BitVec::from_elem(9, false);
        if self.board[disc.x()][disc.y()] != EMPTY {
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
            while self.board[x][y] == -disc.color {
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
            while self.board[x][y] == -disc.color {
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
            while self.board[x][y] == -disc.color {
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
            while self.board[x][y] == -disc.color {
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
            while self.board[x][y] == -disc.color {
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
            while self.board[x][y] == -disc.color {
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
            while self.board[x][y] == -disc.color {
                x = x + 1;
                y = y + 1;
            }
            if self.board[x][y] == disc.color {
                direction.set(Self::DIRECTION_LOWER_RIGHT, true);
            }
        }

        direction
    }

    pub fn move_disc(&mut self, point: &Point) -> bool {
        if point.x < 1 || point.x > Self::SIZE {
            return false;
        }
        if point.y < 1 || point.y > Self::SIZE {
            return false;
        }
        if self.movable_dir[self.turns][point.x][point.y].none() {
            // println!(
            //     "turn: {}, x: {}, y: {}, bit_vec:{:?}",
            //     self.turns, point.x, point.y, self.movable_dir[self.turns][point.x][point.y]
            // );
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
    pub fn is_game_over(&self) -> bool {
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

    pub fn pass(&mut self) -> bool {
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

    pub fn undo(&mut self) -> bool {
        if self.turns == 0 {
            return false;
        }
        self.current_color = -self.current_color;
        let update = self.update_log.pop().expect("it must not be none");

        if update.is_empty() {
            // 前手でpassせざるを得なかったということは、すでに置ける場所はないしdirもすべてNONEなので省略できる気がする
            self.movable_positions[self.turns].clear();
            for x in 1..=(Self::SIZE) {
                for y in 1..=(Self::SIZE) {
                    self.movable_dir[self.turns][x][y] = BitVec::from_elem(9, false);
                }
            }
        } else {
            self.turns = self.turns - 1;
            self.board[update[0].x()][update[0].y()] = EMPTY;
            for i in 1..(update.len()) {
                self.board[update[i].x()][update[i].y()] = -self.current_color;
            }

            let disc_diff = update.len();
            self.discs.set(
                &self.current_color,
                self.discs.get(&self.current_color) - disc_diff as u32,
            );
            self.discs.set(
                &-self.current_color,
                self.discs.get(&-self.current_color) + (disc_diff - 1) as u32,
            );
            self.discs.set(&EMPTY, self.discs.get(&EMPTY) + 1);
        }

        true
    }
    pub fn count_disc(&self, color: &Color) -> u32 {
        *self.discs.get(color)
    }
    pub fn get_color(&self, point: &Point) -> Color {
        self.board[point.x][point.y]
    }
    fn get_movable_positions(&self) -> &Vec<Point> {
        &self.movable_positions[self.turns]
    }
    fn get_update(&self) -> Option<&Vec<Disc>> {
        self.update_log.last()
    }
    fn get_current_color(&self) -> Color {
        self.current_color
    }
    fn get_turns(&self) -> usize {
        self.turns
    }
}
