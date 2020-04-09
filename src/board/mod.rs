pub mod pieces;

pub enum Color {
    Black,
    White
}

pub struct Board {
    pieces: [pieces::Piece; 64],
    side: Color,
    fifty_moves: u8,
}
