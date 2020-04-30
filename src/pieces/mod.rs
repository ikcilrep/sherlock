use crate::board::Board;
use crate::moves::Move;
use crate::pieces::color::uncolorize_piece;

pub mod bishop;
pub mod color;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;
pub mod sliders;

pub type ColorizedPiece = u8;
pub type Piece = u8;
pub type Generator = fn(usize, &Board, &mut Vec<Move>);
pub const WHITE_PAWN: ColorizedPiece = 0;
pub const BLACK_PAWN: ColorizedPiece = 1;
pub const WHITE_ROOK: ColorizedPiece = 2;
pub const BLACK_ROOK: ColorizedPiece = 3;
pub const WHITE_KNIGHT: ColorizedPiece = 4;
pub const BLACK_KNIGHT: ColorizedPiece = 5;
pub const WHITE_BISHOP: ColorizedPiece = 6;
pub const BLACK_BISHOP: ColorizedPiece = 7;
pub const WHITE_QUEEN: ColorizedPiece = 8;
pub const BLACK_QUEEN: ColorizedPiece = 9;
pub const WHITE_KING: ColorizedPiece = 10;
pub const BLACK_KING: ColorizedPiece = 11;
pub const EMPTY_SQUARE: ColorizedPiece = 12;

pub const PAWN: Piece = 0;
pub const ROOK: Piece = 1;
pub const KNIGHT: Piece = 2;
pub const BISHOP: Piece = 3;
pub const QUEEN: Piece = 4;
pub const KING: Piece = 5;

pub const PSEUDO_LEGAL_MOVE_GENERATORS: [Generator; 7] = [
    pawn::generate_pseudo_legal_moves,
    rook::generate_pseudo_legal_moves,
    knight::generate_pseudo_legal_moves,
    bishop::generate_pseudo_legal_moves,
    queen::generate_pseudo_legal_moves,
    king::generate_pseudo_legal_moves,
    |_, _, _| {},
];

#[inline]
pub fn generate_all_pseudo_legal_moves(board: &Board, result: &mut Vec<Move>) {
    for (from, piece) in board.pieces.iter().enumerate() {
        PSEUDO_LEGAL_MOVE_GENERATORS[uncolorize_piece(*piece) as usize](from, board, result);
    }
}
