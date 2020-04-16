use crate::board::pieces::color::{get_piece_color, Color};
use crate::board::pieces::EMPTY_SQUARE;
pub mod moves;
pub mod pieces;

pub struct Board {
    pub pieces: [pieces::Piece; 64],
    side: Color,
    fifty_moves: u8,
    pub has_king_stayed_in_place: [bool; 2],
    pub has_queens_rook_stayed_in_place: [bool; 2],
    pub has_kings_rook_stayed_in_place: [bool; 2],
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [
                pieces::WHITE_ROOK,
                pieces::WHITE_KNIGHT,
                pieces::WHITE_BISHOP,
                pieces::WHITE_QUEEN,
                pieces::WHITE_KING,
                pieces::WHITE_BISHOP,
                pieces::WHITE_KNIGHT,
                pieces::WHITE_ROOK,
                pieces::WHITE_PAWN,
                pieces::WHITE_PAWN,
                pieces::WHITE_PAWN,
                pieces::WHITE_PAWN,
                pieces::WHITE_PAWN,
                pieces::WHITE_PAWN,
                pieces::WHITE_PAWN,
                pieces::WHITE_PAWN,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::EMPTY_SQUARE,
                pieces::BLACK_PAWN,
                pieces::BLACK_PAWN,
                pieces::BLACK_PAWN,
                pieces::BLACK_PAWN,
                pieces::BLACK_PAWN,
                pieces::BLACK_PAWN,
                pieces::BLACK_PAWN,
                pieces::BLACK_PAWN,
                pieces::BLACK_ROOK,
                pieces::BLACK_KNIGHT,
                pieces::BLACK_BISHOP,
                pieces::BLACK_QUEEN,
                pieces::BLACK_KING,
                pieces::BLACK_BISHOP,
                pieces::BLACK_KNIGHT,
                pieces::BLACK_ROOK,
            ],
            side: pieces::color::WHITE,
            fifty_moves: 0,
            has_king_stayed_in_place: [true, true],
            has_queens_rook_stayed_in_place: [true, true],
            has_kings_rook_stayed_in_place: [true, true],
        }
    }

    #[inline]
    pub fn can_be_moved(self: &Board, to: usize, piece_to_move_color: Color) -> bool {
        self.pieces[to] == EMPTY_SQUARE || get_piece_color(self.pieces[to]) != piece_to_move_color
    }
}
