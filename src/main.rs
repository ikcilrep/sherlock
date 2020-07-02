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
use crate::game::tree::Tree;
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

fn make_move(move_str: &str, board: &mut Board) {
    let half_move = from_algebraic_notation(&String::from(move_str), board).unwrap();
    board.make_move(half_move);
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut board = Board::new();
    let mut counter = 1;

    let mut tmp_tree = Box::new(Tree::new(&mut board));
    let mut tree = &mut tmp_tree;

    loop {
        if handle_game_result(&mut board) {
            break;
        }

        for _ in 0..10000 {
            tree.make_round(&mut rng);
        }

        let half_move = tree.get_best_move().unwrap();
        let move_str = to_algebraic_notation(half_move, &mut board);
        board.make_move(half_move);
        match tree.get_subtree(half_move) {
            Some(subtree) => tree = subtree,
            None => {
                tmp_tree = Box::new(Tree::new(&mut board));
                tree = &mut tmp_tree
            }
        };
        println!("{}.\t{}", counter, move_str);
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
                            match tree.get_subtree(half_move) {
                                Some(subtree) => tree = subtree,
                                None => {
                                    tmp_tree = Box::new(Tree::new(&mut board));
                                    tree = &mut tmp_tree
                                }
                            };
                            break;
                        }
                        None => {
                            println!("Incorrect move, try again.");
                            opponent_move_str.clear();
                        }
                    }
                }
                Err(message) => println!("{}", message),
            }
        }
        counter += 1;
    }
}
