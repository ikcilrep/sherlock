pub mod moves;
pub mod pieces;
pub mod squares;

pub enum Color {
    Black,
    White,
}

pub struct Board {
    pieces: [pieces::Piece; 64],
    side: Color,
    fifty_moves: u8,
}
