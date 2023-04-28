use std::io::{self, Write};

fn main() {
    println!("Welcome to Connect 4! player 1: X, player 2: O\n");
    let mut running = true;
    while running == true {
        let mut gb = Board::build();
        let mut over = GameStates::Playing;
        gb.show_board();
        while over == GameStates::Playing {
            let p_num = match gb.p1_turn {
                true => String::from("1"),
                false => String::from("2"),
            };
            let mut dropped = false;
            while dropped == false {
                print!("Player {} please pick a column 1-4: ", p_num);
                io::stdout().flush().expect("failed to flush.");
                let mut col: String = String::new();
                io::stdin()
                    .read_line(&mut col)
                    .expect("Failed to read line");
                let col: i32 = match col.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 5,
                };
                if (1..=4).contains(&col) == false {
                    println!("You must pick a number from One to Four. Try again!");
                } else {
                    dropped = gb.drop_token(col);
                    if dropped == false {
                        println!("You cannot drop a token there. Try again!");
                    }
                }
                over = gb.win_check();
            }
            gb.show_board();
        }
        if over == GameStates::Cats {
            println!("Cats eye. No one won!");
            running = false;
        } else {
            match gb.p1_turn {
                true => {
                    println!("Player 2 Won!");
                    running = false;
                }
                false => {
                    println!("Player 1 Won!");
                    running = false;
                }
            }
        }
    }
}

const WIN_CHECKS: [[usize; 4]; 10] = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [8, 9, 10, 11],
    [12, 13, 14, 15],
    [0, 4, 8, 12],
    [1, 5, 9, 13],
    [2, 6, 10, 14],
    [3, 7, 11, 15],
    [0, 5, 10, 15],
    [3, 6, 9, 12],
];

#[derive(PartialEq, Debug)]
enum GameStates {
    Over = 1,
    Playing = 2,
    Cats = 3,
}

struct Board {
    game_board: Vec<char>,
    p1_token: char,
    p2_token: char,
    col_heights: Vec<i32>,
    p1_turn: bool,
}

impl Board {
    fn build() -> Board {
        Board {
            game_board: vec!['*'; 16],
            p1_token: 'X',
            p2_token: 'O',
            col_heights: vec![3; 4],
            p1_turn: true,
        }
    }
    fn get_spot(&self, col: i32) -> i32 {
        return 0 + (col - 1) + (4 * self.col_heights[(col - 1) as usize]);
    }
    fn show_board(&self) {
        for i in 0..4 {
            println!(
                "{} {} {} {}",
                self.game_board[0 + (4 * i)],
                self.game_board[1 + (4 * i)],
                self.game_board[2 + (4 * i)],
                self.game_board[3 + (4 * i)]
            )
        }
        println!("\n1 2 3 4");
    }
    fn drop_token(&mut self, col: i32) -> bool {
        if (self.col_heights[(col - 1) as usize]) < 0 {
            return false;
        }
        let index: i32 = self.get_spot(col);
        let token = match self.p1_turn {
            true => self.p1_token,
            false => self.p2_token,
        };
        self.game_board[index as usize] = token;
        self.col_heights[(col - 1) as usize] -= 1;
        self.p1_turn = !self.p1_turn;
        return true;
    }
    fn win_check(&mut self) -> GameStates {
        self.p1_turn = !self.p1_turn;
        let token = match self.p1_turn {
            true => self.p1_token,
            false => self.p2_token,
        };
        for set in WIN_CHECKS {
            if self.game_board[set[0]] == token
                && self.game_board[set[1]] == token
                && self.game_board[set[2]] == token
                && self.game_board[set[3]] == token
            {
                self.p1_turn = !self.p1_turn;
                return GameStates::Over;
            }
        }
        if self.game_board.iter().any(|&i| i == '*') {
            self.p1_turn = !self.p1_turn;
            return GameStates::Playing;
        } else {
            self.p1_turn = !self.p1_turn;
            return GameStates::Cats;
        }
    }
}
