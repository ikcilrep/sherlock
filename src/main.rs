mod board;
use crate::board::Board;

fn main() {
    let mut board = Board::new();
    let mut moves = Vec::new();
    let t: i8 = -1;
    println!("{}", (t as usize));
    board.pieces[8] = board::pieces::EMPTY_SQUARE;
    board.pieces[11] = board::pieces::EMPTY_SQUARE;
    board.pieces[9] = board::pieces::EMPTY_SQUARE;
    board::pieces::rook::generate_pseudo_legal_rook_moves(0, &board, &mut moves);
    board::pieces::bishop::generate_pseudo_legal_bishop_moves(2, &board, &mut moves);
    board::pieces::knight::generate_pseudo_legal_knight_moves(1, &board, &mut moves);
    for m in moves {
        println!("Move: {0:b} {0}", (m >> 14) & 63);
    }
}
