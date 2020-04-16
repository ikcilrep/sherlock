pub mod bishop;
pub mod color;
pub mod king;
pub mod knight;
pub mod queen;
pub mod rook;
pub mod sliders;

pub type Piece = i8;
pub const WHITE_PAWN: Piece = 0;
pub const BLACK_PAWN: Piece = 1;
pub const WHITE_ROOK: Piece = 2;
pub const BLACK_ROOK: Piece = 3;
pub const WHITE_KNIGHT: Piece = 4;
pub const BLACK_KNIGHT: Piece = 5;
pub const WHITE_BISHOP: Piece = 6;
pub const BLACK_BISHOP: Piece = 7;
pub const WHITE_QUEEN: Piece = 8;
pub const BLACK_QUEEN: Piece = 9;
pub const WHITE_KING: Piece = 10;
pub const BLACK_KING: Piece = 11;
pub const EMPTY_SQUARE: Piece = 12;

pub const PAWN: Piece = 0;
pub const ROOK: Piece = 1;
pub const KNIGHT: Piece = 2;
pub const BISHOP: Piece = 3;
pub const QUEEN: Piece = 4;
pub const KING: Piece = 5;
