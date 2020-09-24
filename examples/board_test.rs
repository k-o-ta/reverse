use reversi::reversi::board::Board;
use reversi::reversi::{Point, BLACK, EMPTY, WHITE};

fn main() {
    let mut console = Console::new();
    loop {
        console.print();
        println!(
            "黒石{} 白石{} 空マス{}",
            console.0.count_disc(&BLACK),
            console.0.count_disc(&WHITE),
            console.0.count_disc(&EMPTY)
        );
        println!("手を入力してください");
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        s = s.trim().to_string();

        if s == "p" {
            if !console.0.pass() {
                println!("パスできません\n");
            }
            continue;
        }
        if s == "u" {
            console.0.undo();
            continue;
        }
        match Point::from(&s) {
            Err(e) => {
                println!("リバーシ形式の手を入力してください: {}\n", e);
                continue;
            }
            Ok(point) => {
                if !console.0.move_disc(&point) {
                    println!("そこには置けません: {}\n", point);
                    continue;
                }
                if console.0.is_game_over() {
                    println!("-------------ゲーム終了-------------");
                    return;
                }
            }
        }
    }
    return;
}

struct Console(Board);

impl Console {
    fn new() -> Console {
        Console(Board::new())
    }

    fn print(&self) {
        println!("   a b c d e f g h  ");
        for y in 1..=8 {
            print!(" {}", y);
            for x in 1..=8 {
                match self.0.get_color(&Point::new(x, y)) {
                    BLACK => {
                        print!(" o");
                    }
                    WHITE => {
                        print!(" x");
                    }
                    _ => {
                        print!("  ");
                    }
                }
            }
            println!("");
        }
    }
}
