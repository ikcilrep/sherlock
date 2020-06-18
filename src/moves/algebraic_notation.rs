// This module doesn't have to be fast.
extern crate regex;

use crate::board::Board;
use crate::moves::constructors::new_promotion;
use crate::moves::{
    get_captured_piece, get_from, get_move_type, get_moved_piece, get_to, Move,
    CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE, EN_PASSANT, NORMAL_MOVE,
};
use crate::pieces::color::{colorize_piece, get_piece_color, uncolorize_piece};
use crate::pieces::pawn::PAWN_STEPS;
use crate::pieces::{ColorizedPiece, Piece, BISHOP, EMPTY_SQUARE, KING, KNIGHT, PAWN, QUEEN, ROOK};

use regex::Regex;

fn file_to_char(file: u8) -> char {
    (file + 97) as char
}

fn rank_to_char(rank: u8) -> char {
    (rank + 1).to_string().as_bytes()[0] as char
}

fn char_to_rank(rank: char) -> i8 {
    rank as i8 - 1
}

fn char_to_file(file: char) -> i8 {
    file as i8 - 97
}

fn string_to_location(location_string: &str) -> i8 {
    let file = char_to_file(location_string.chars().nth(0).unwrap());
    let rank = char_to_rank(location_string.chars().nth(1).unwrap());
    (rank << 3) + file
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

fn first_char(string: &str) -> char {
    string.chars().next().unwrap()
}

fn get_piece_from_string(piece_string: &str, board: &Board) -> ColorizedPiece {
    colorize_piece(
        char_to_piece(first_char(piece_string)).unwrap(),
        board.state.side,
    )
}

pub fn from_algebraic_notation(move_string: &String, board: &mut Board) -> Option<Move> {
    lazy_static! {
        static ref PAWN_MOVE: Regex = Regex::new(
            "^(?:(?P<file>[a-e])x)?(?P<to>[a-e][1-8])(?:=(?P<promoted_piece>[KQRNB]))?$"
        )
        .unwrap();
        static ref NOT_PAWN_MOVE: Regex =
            Regex::new("^(?<moved_piece>[KQRNB])x?(?P<to>[a-e][1-8])$").unwrap();
    }
    if PAWN_MOVE.is_match(move_string) {
        let moved_piece = colorize_piece(PAWN, board.state.side);
        let captures = PAWN_MOVE.captures(move_string).unwrap();
        let to = string_to_location(captures.name("to").unwrap().as_str());
        let promoted_piece = captures
            .name("promoted_piece")
            .map(|promoted_piece_capture| {
                get_piece_from_string(promoted_piece_capture.as_str(), board)
            })
            .unwrap_or(moved_piece);
        let from = match captures.name("file") {
            Some(file_capture) => {
                let file = char_to_file(first_char(file_capture.as_str()));
                to - (to & 7) - PAWN_STEPS[board.state.side as usize][0] + file
            }
            None => to - PAWN_STEPS[board.state.side as usize][0],
        };

        let distance = (to - from).abs();

        if board.is_square_on_board(from)
            && (distance >= 7 && distance <= 9)
            && board.state.pieces[from as usize] == moved_piece
            && (distance == 8) == (board.state.pieces[to as usize] == EMPTY_SQUARE)
            && !board.is_piece_pinned(
                from,
                to,
                board.state.king_positions[board.state.side as usize],
            )
        {
            return Some(new_promotion(from as usize, to, promoted_piece, board));
        }
    }

    None
}
