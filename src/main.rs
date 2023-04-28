// use std::io::{self, Write};

// fn main() {
//     print!("Enter a number 1-4: ");
//     io::stdout().flush().unwrap();

//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     let y: i32 = guess.trim().parse().expect("Not a number");

//     let col_height = 2;

//     let c: i32 = 0 + (y - 1) + (4*col_height);



//     println!("{}",c);
// }

#![allow(dead_code)]
fn main(){
    let gb = Board::build();
    gb.show_board();
    gb.show_board();
}

struct Board{
    game_board: Vec<char>,
    p1_token: char,
    p2_token: char,
    col_heights: Vec<i32>,
    p1_turn: bool,
}

impl Board{
    fn build()->Board{
        Board{
            game_board: vec!['*';16],
            p1_token: 'X',
            p2_token: 'O',
            col_heights: vec![0; 4],
            p1_turn: true,
        }
    }
    fn get_spot(&self, col: i32)->i32{
        return 0 + (col-1) + (4*self.col_heights[(col-1)as usize]);
    }
    fn show_board(&self){
        for i in 1..=4{
            println!("{} {} {} {}",self.game_board[0*i],self.game_board[1*i],self.game_board[2*i],self.game_board[3*i])
        }
    }
}