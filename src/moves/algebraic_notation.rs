// This module doesn't have to be fast.

use crate::board::Board;
use crate::moves::{
    get_captured_piece, get_from, get_move_type, get_moved_piece, get_to, Move,
    CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE, EN_PASSANT, NORMAL_MOVE,
};
use crate::pieces::color::{colorize_piece, get_piece_color, uncolorize_piece};
use crate::pieces::{ColorizedPiece, Piece, BISHOP, EMPTY_SQUARE, KING, KNIGHT, PAWN, QUEEN, ROOK};

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

fn char_to_piece(piece: char) -> Option<Piece> {
    match piece {
        'B' => Some(BISHOP),
        'K' => Some(KING),
        'N' => Some(KNIGHT),
        'Q' => Some(QUEEN),
        'R' => Some(ROOK),
        _ => None,
    }
}

fn remove_ambiguities(half_move: Move, move_string_part: &mut String, board: &Board) {
    let from = get_from(half_move) as i8;
    let moved_piece = get_moved_piece(half_move);
    let attacked_color = get_piece_color(moved_piece);
    let to = get_to(half_move);
    let attackers_locations =
        board.get_pieces_of_type_attacking_square_locations(to as i8, moved_piece, attacked_color);

    for attacker_location in attackers_locations {
        if attacker_location != from {
            let attacker_location_rank = attacker_location >> 3;
            let from_rank = from >> 3;
            let attacker_location_file = attacker_location & 7;
            let from_file = from & 7;

            if attacker_location_rank == from_rank {
                move_string_part.push(file_to_char(from_file as u8));
            } else if attacker_location_file == from_file {
                move_string_part.push(rank_to_char(from_rank as u8));
            } else {
                move_string_part.push(file_to_char(from_file as u8));
            }
            break;
        }
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
            let is_capture = get_captured_piece(half_move) != EMPTY_SQUARE;
            if uncolorized_moved_piece != PAWN {
                result.push(piece_to_char(uncolorized_moved_piece));
                remove_ambiguities(half_move, &mut result, board);
            } else if is_capture {
                let from_file = (get_from(half_move) & 7) as u8;
                result.push(file_to_char(from_file));
            }
            // Rank or file will be here.
            if is_capture {
                result.push('x');
            }

            let to = get_to(half_move) as u8;
            result.push_str(location_to_string(to).as_str());

            result
        }
        _ => panic!("Invalid move type."),
    }
}

fn get_piece_from_move_string(move_string: &String, board: &Board) -> Option<ColorizedPiece> {
    move_string
        .chars()
        .next()
        .and_then(|first_char| {
            if first_char.is_uppercase() {
                char_to_piece(first_char)
            } else {
                Some(PAWN)
            }
        })
        .map(|piece| colorize_piece(piece, board.state.side))
}

pub fn from_algebraic_notaton(move_string: &String, board: &Board) -> Option<Move> {
    match get_piece_from_move_string(move_string, board) {
        Some(_) => {}
        None => return None,
    };

    None
}
