#![allow(dead_code)]
#![allow(unused_imports)]
use std::io;

fn main(){
    let mut gb = Board::build();
    gb.show_board();

    gb.drop_token(1);
    gb.drop_token(2);
    gb.drop_token(1);
    gb.drop_token(2);
    gb.drop_token(1);
    gb.drop_token(2);
    gb.drop_token(1);

    gb.show_board();

    println!("{}",gb.win_check());
}

const WIN_CHECKS: [[usize; 4];10] = [
    [0,1,2,3],
    [4,5,6,7],
    [8,9,10,11],
    [12,13,14,15],
    [0,4,8,12],
    [1,5,9,13],
    [2,6,10,14],
    [3,7,11,15],
    [0,5,10,15],
    [3,6,9,12]
];

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
            col_heights: vec![3; 4],
            p1_turn: true,
        }
    }
    fn get_spot(&self, col: i32)->i32{
        return 0 + (col-1) + (4*self.col_heights[(col-1)as usize]);
    }
    fn show_board(&self){
        for i in 0..4{
            println!("{} {} {} {}",self.game_board[0+(4*i)],self.game_board[1+(4*i)],self.game_board[2+(4*i)],self.game_board[3+(4*i)])
        }
    }
    fn drop_token(&mut self, col: i32){
        let index: i32 = self.get_spot(col);
        let token = match self.p1_turn {
            true => self.p1_token,
            false => self.p2_token,
        };
        self.game_board[index as usize] = token;
        self.col_heights[(col-1) as usize] -= 1;
        self.p1_turn = !self.p1_turn;
    }
    fn win_check(&mut self)->bool{
        let mut win = false;
        self.p1_turn = !self.p1_turn;
        let token = match self.p1_turn {
            true => self.p1_token,
            false => self.p2_token,
        };
        for set in WIN_CHECKS{
            if self.game_board[set[0]] == token && self.game_board[set[1]] == token && self.game_board[set[2]] == token && self.game_board[set[3]] == token {
                win = true;
            }
        }
        return win;
    }
}