#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod board;
mod moves;
mod pieces;
use crate::board::Board;
use crate::moves::algebraic_notation::{from_algebraic_notation, to_algebraic_notation};
use crate::pieces::generate_all_pseudo_legal_moves;
use crate::pieces::{BLACK_KNIGHT, EMPTY_SQUARE};

fn main() {
    let mut board = Board::new();
    let mut moves = Vec::new();
    // let t: i8 = -1;
    // println!("{}", (t as usize));
    // board.state.pieces[8] = EMPTY_SQUARE;
    // board.state.pieces[11] = EMPTY_SQUARE;
    // board.state.pieces[12] = EMPTY_SQUARE;
    // board.state.pieces[10] = EMPTY_SQUARE;
    // board.state.pieces[9] = EMPTY_SQUARE;
    // board::pieces::bishop::generate_pseudo_legal_bishop_moves(2, &board, &mut moves);
    // board::pieces::knight::generate_pseudo_legal_knight_moves(1, &board, &mut moves);
    // board::pieces::queen::generate_pseudo_legal_queen_moves(3, &board, &mut moves);
    board.state.pieces[8] = EMPTY_SQUARE;
    board.state.pieces[1] = EMPTY_SQUARE;
    board.state.pieces[2] = EMPTY_SQUARE;
    board.state.pieces[3] = EMPTY_SQUARE;
    board.state.pieces[11] = EMPTY_SQUARE;
    board.state.pieces[12] = EMPTY_SQUARE;
    board.state.pieces[17] = BLACK_KNIGHT;

    generate_all_pseudo_legal_moves(&board, &mut moves);
    println!(
        "{}",
        to_algebraic_notation(
            from_algebraic_notation(&String::from("Ra6"), &mut board).unwrap(),
            &mut board
        )
    );
    for m in moves {
        println!("{}", to_algebraic_notation(m, &mut board));
    }
}
