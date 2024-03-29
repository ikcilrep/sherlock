extern crate rand;

use crate::board::Board;
use crate::moves::Move;
use crate::pieces::color::{get_piece_color, uncolorize_piece};

use rand::rngs::ThreadRng;
use rand::Rng;

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

pub const RANDOM_LEGAL_MOVE_GENERATORS: [fn(usize, &mut Board, &mut ThreadRng) -> Option<Move>; 7] = [
    pawn::generate_random_legal_move,
    rook::generate_random_legal_move,
    knight::generate_random_legal_move,
    bishop::generate_random_legal_move,
    queen::generate_random_legal_move,
    king::generate_random_legal_move,
    |_, _, _| None,
];

impl Board {
    pub fn generate_all_pseudo_legal_moves(&self, result: &mut Vec<Move>) {
        self.state
            .pieces
            .iter()
            .enumerate()
            .filter(|&(_, &piece)| get_piece_color(piece) == self.state.side)
            .for_each(|(from, &piece)| {
                PSEUDO_LEGAL_MOVE_GENERATORS[uncolorize_piece(piece) as usize](from, self, result)
            });
    }

    // Temporary, naive version.
    pub fn generate_all_legal_moves(&mut self) -> Vec<Move> {
        let mut result = Vec::new();
        self.generate_all_pseudo_legal_moves(&mut result);
        result
            .iter()
            .cloned()
            .filter(|&half_move| self.is_move_legal(half_move))
            .collect()
    }

    pub fn generate_random_legal_move(&mut self, rng: &mut ThreadRng) -> Move {
        let start = rng.gen_range(0, 64);
        let mut i = start;
        while {
            let piece = self.state.pieces[i];
            if get_piece_color(piece) == self.state.side {
                let uncolorized_piece = uncolorize_piece(piece) as usize;
                match RANDOM_LEGAL_MOVE_GENERATORS[uncolorized_piece](i, self, rng) {
                    Some(half_move) => return half_move,
                    None => {}
                }
            }
            i += 1;
            i &= 63;
            i != start
        } {}
        panic!("Can't find any legal move.");
    }
}
