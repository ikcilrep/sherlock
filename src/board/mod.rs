pub mod moves;
pub mod pieces;

pub type Color = i8;
pub const WHITE: Color = 0;
pub const BLACK: Color = 1;

pub struct Board {
    pub pieces: [pieces::Piece; 64],
    side: Color,
    fifty_moves: u8,
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
            side: WHITE,
            fifty_moves: 0,
        }
    }
}
