#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod board;
mod game;
mod moves;
mod pieces;

use crate::board::Board;
use crate::game::result::GameResult;
use crate::moves::algebraic_notation::from_algebraic_notation;
use crate::moves::algebraic_notation::to_algebraic_notation;
use crate::pieces::color::WHITE;

fn handle_game_result(board: &mut Board) -> bool {
    let king_attackers_locations = board.get_attackers_of_king_square_locations(board.state.side);

    match board.get_game_state(&king_attackers_locations) {
        GameResult::StillInProgress => false,
        GameResult::Win(color) => {
            if color == WHITE {
                println!("White won!");
            } else {
                println!("Black won!");
            }
            true
        }
        GameResult::Draw => {
            println!("Draw!");
            true
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut board = Board::new();
    let mut counter = 1;
    loop {
        let half_move = board.get_best_move(1000, &mut rng);
        board.make_move(half_move);
        println!(
            "{}.\t{}",
            counter,
            to_algebraic_notation(half_move, &mut board)
        );
        if handle_game_result(&mut board) {
            break;
        }
        let mut opponent_move_str = String::new();
        loop {
            match std::io::stdin().read_line(&mut opponent_move_str) {
                Ok(_) => {
                    opponent_move_str.pop();
                    match from_algebraic_notation(&opponent_move_str, &mut board) {
                        Some(opponent_move) => {
                            board.make_move(opponent_move);
                            if handle_game_result(&mut board) {
                                break;
                            }
                            break;
                        }
                        None => println!("Incorrect move, try again."),
                    }
                }
                Err(message) => println!("{}", message),
            }
        }
        counter += 1;
    }
}
