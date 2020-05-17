// This module doesn't have to be fast.

use crate::board::Board;
use crate::moves::{
    get_captured_piece, get_from, get_move_type, get_moved_piece, get_to, Move,
    CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE, EN_PASSANT, NORMAL_MOVE,
};
use crate::pieces::color::{get_piece_color, uncolorize_piece};
use crate::pieces::{Piece, BISHOP, EMPTY_SQUARE, KING, KNIGHT, PAWN, QUEEN, ROOK};

fn file_to_char(file: u8) -> char {
    (file + 97) as char
}

fn rank_to_char(rank: u8) -> char {
    (rank + 1).to_string().as_bytes()[0] as char
}

fn location_to_string(location: u8) -> String {
    let file = location & 7;
    let rank = location >> 3;
    let mut result = String::with_capacity(2);
    result.push(file_to_char(file));
    result.push(rank_to_char(rank));
    result
}

fn piece_to_char(piece: Piece) -> char {
    match piece {
        BISHOP => 'B',
        KING => 'K',
        KNIGHT => 'N',
        QUEEN => 'Q',
        ROOK => 'R',
        _ => panic!("Invalid piece type."),
    }
}

pub fn to_algebraic_notation(half_move: Move, board: &Board) -> String {
    match get_move_type(half_move) {
        CASTLING_KINGS_SIDE => String::from("O-O"),
        CASTLING_QUEENS_SIDE => String::from("O-O-O"),
        EN_PASSANT | NORMAL_MOVE => {
            let mut result = String::new();
            let moved_piece = get_moved_piece(half_move);
            let uncolorized_moved_piece = uncolorize_piece(moved_piece);
            let from = get_from(half_move) as i8;
            let attacked_color = get_piece_color(moved_piece);
            if uncolorized_moved_piece != PAWN {
                result.push(piece_to_char(uncolorized_moved_piece));

                let to = get_to(half_move);
                let attackers_locations = board.get_pieces_of_type_attacking_square_locations(
                    to as i8,
                    moved_piece,
                    attacked_color,
                );

                for attacker_location in attackers_locations {
                    if attacker_location != from {
                        let attacker_location_rank = attacker_location >> 3;
                        let from_rank = from >> 3;
                        let attacker_location_file = attacker_location & 7;
                        let from_file = from & 7;

                        if attacker_location_rank == from_rank {
                            result.push(file_to_char(from_file as u8));
                        } else if attacker_location_file == from_file {
                            result.push(rank_to_char(from_rank as u8));
                        } else {
                            result.push(file_to_char(from_file as u8));
                        }
                        break;
                    }
                }
            }
            // Rank or file will be here.
            if get_captured_piece(half_move) != EMPTY_SQUARE {
                result.push('x');
            }
            result.push_str(location_to_string(get_to(half_move) as u8).as_str());

            result
        }
        _ => panic!("Invalid move type."),
    }
}
