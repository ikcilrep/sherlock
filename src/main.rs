mod board;
mod moves;
mod pieces;
use crate::board::Board;
use crate::pieces::EMPTY_SQUARE;
use crate::pieces::{king, rook};

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

    rook::generate_pseudo_legal_rook_moves(0, &board, &mut moves);
    king::generate_pseudo_legal_king_moves(4, &board, &mut moves);
    for m in moves {
        println!("Move: {:026b} {0}", (m >> 14) & 63);
    }
}
