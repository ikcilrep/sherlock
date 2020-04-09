pub enum Piece {
    EmptySquare1,
    EmptySquare2, // it's for optimization: (captured piece << 1) + color
    WhitePawn,
    BlackPawn,
    WhiteRook,
    BlackRook,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}
