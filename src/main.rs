mod board;
mod moves;
mod pieces;
extern crate rand;
use crate::board::Board;
use crate::moves::algebraic_notation::to_algebraic_notation;
use crate::pieces::generate_all_pseudo_legal_moves;
use crate::pieces::{bishop, king, knight, pawn, queen, rook, BLACK_KNIGHT, EMPTY_SQUARE};
use rand::thread_rng;

fn main() {
    let mut board = Board::new();
    let mut moves = Vec::new();
    // let t: i8 = -1;
    // println!("{}", (t as usize));
    // board.pieces[8] = EMPTY_SQUARE;
    // board.pieces[11] = EMPTY_SQUARE;
    // board.pieces[12] = EMPTY_SQUARE;
    // board.pieces[10] = EMPTY_SQUARE;
    // board.pieces[9] = EMPTY_SQUARE;
    // board::pieces::bishop::generate_pseudo_legal_bishop_moves(2, &board, &mut moves);
    // board::pieces::knight::generate_pseudo_legal_knight_moves(1, &board, &mut moves);
    // board::pieces::queen::generate_pseudo_legal_queen_moves(3, &board, &mut moves);
    board.pieces[8] = EMPTY_SQUARE;
    board.pieces[1] = EMPTY_SQUARE;
    board.pieces[2] = EMPTY_SQUARE;
    board.pieces[3] = EMPTY_SQUARE;
    board.pieces[11] = EMPTY_SQUARE;
    board.pieces[12] = EMPTY_SQUARE;
    //board.pieces[17] = BLACK_KNIGHT;
    generate_all_pseudo_legal_moves(&board, &mut moves);
    for m in moves {
        println!("{}", to_algebraic_notation(m, &board));
    }
}
