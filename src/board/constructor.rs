use crate::board::board_state::BoardState;
use crate::board::Board;
use crate::pieces;

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
            state: BoardState::new(),
            last_state: BoardState::new(),
            pieces_count: 32,
        }
    }
}
