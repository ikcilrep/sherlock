mod board;
mod moves;
mod pieces;
extern crate rand;
use crate::board::Board;
use crate::pieces::color::get_piece_color;
use crate::pieces::generate_all_pseudo_legal_moves;
use crate::pieces::{king, knight, pawn, BLACK_KNIGHT, EMPTY_SQUARE};
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
    board.pieces[1] = EMPTY_SQUARE;
    board.pieces[2] = EMPTY_SQUARE;
    board.pieces[3] = EMPTY_SQUARE;
    board.pieces[17] = BLACK_KNIGHT;
    generate_all_pseudo_legal_moves(&board, &mut moves);
    let mut rng = thread_rng();
    println!(
        "{}",
        moves::get_to(knight::generate_random_pseudo_legal_move(
            1, &board, &mut rng
        ))
    );
    println!(
        "{}",
        moves::get_to(king::generate_random_pseudo_legal_move(4, &board, &mut rng))
    );

    println!(
        "{}",
        moves::get_to(pawn::generate_random_pseudo_legal_move(8, &board, &mut rng))
    );

    // for m in moves {
    //     println!("From: {0}", moves::get_from(m));
    //     println!("To: {0}", moves::get_to(m));
    //     println!("Captured: {0}", moves::get_captured_piece(m));
    //     println!("Promoted: {0}", moves::get_promoted_piece(m));
    //     println!("Moved: {0}", moves::get_moved_piece(m));
    // }
}
