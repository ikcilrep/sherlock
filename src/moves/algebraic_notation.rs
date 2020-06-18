// This module doesn't have to be fast.
extern crate regex;

use crate::board::Board;
use crate::moves::constructors::{new_move, new_promotion};
use crate::moves::{
    get_captured_piece, get_from, get_move_type, get_moved_piece, get_to, Move,
    CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE, EN_PASSANT, NORMAL_MOVE,
};
use crate::pieces::color::{colorize_piece, uncolorize_piece};
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
    rank.to_string().parse::<i8>().unwrap() - 1
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

fn remove_ambiguities(half_move: Move, move_string_part: &mut String, board: &mut Board) {
    let from = get_from(half_move) as i8;
    let moved_piece = get_moved_piece(half_move);
    let to = get_to(half_move);
    let attackers_locations =
        board.get_pieces_of_type_defending_square_locations(to as i8, moved_piece);
    // more ambiguous attackers in future
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

pub fn to_algebraic_notation(half_move: Move, board: &mut Board) -> String {
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

fn get_unambiguous_from(
    candidate_froms: &Vec<i8>,
    unambiguity: Option<&str>,
    piece_to_move: ColorizedPiece,
    board: &mut Board,
) -> Option<i8> {
    let find_unambiguous_from = || {
        let unambiguous_froms: Vec<i8> = candidate_froms
            .iter()
            .filter(|&&from| board.state.pieces[from as usize] == piece_to_move)
            .map(|&from| from)
            .collect();
        if unambiguous_froms.len() == 1 {
            Some(unambiguous_froms[0])
        } else {
            None
        }
    };
    match unambiguity {
        Some(unambiguity) => match unambiguity.len() {
            1 => {
                let first_char = first_char(unambiguity);
                let unambiguous_froms: Vec<i8> = if first_char.is_digit(10) {
                    let rank = char_to_rank(first_char);
                    candidate_froms
                        .iter()
                        .filter(|&&from| from >> 3 == rank)
                        .map(|&from| from)
                        .collect()
                } else {
                    let file = char_to_file(first_char);
                    candidate_froms
                        .iter()
                        .filter(|&&from| from & 7 == file)
                        .map(|&from| from)
                        .collect()
                };

                if unambiguous_froms.len() == 1 {
                    Some(unambiguous_froms[0])
                } else {
                    None
                }
            }
            2 => {
                let location = string_to_location(unambiguity);
                if candidate_froms.contains(&location) {
                    Some(location)
                } else {
                    None
                }
            }
            0 => find_unambiguous_from(),
            _ => None,
        },
        None => find_unambiguous_from(),
    }
}

pub fn from_algebraic_notation(move_string: &String, board: &mut Board) -> Option<Move> {
    lazy_static! {
        static ref PAWN_MOVE: Regex = Regex::new(
            "^(?:(?P<file>[a-e])x)?(?P<to>[a-e][1-8])(?:=(?P<piece_after_promotion>[KQRNB]))?$"
        )
        .unwrap();
        static ref NOT_PAWN_MOVE: Regex = Regex::new(
            "^(?P<piece_to_move>[KQRNB])(?P<unambiguity>[a-e]?[1-8]?)?x?(?P<to>[a-e][1-8])$"
        )
        .unwrap();
    }

    let king_location = board.state.king_positions[board.state.side as usize];
    // en passant, castling in future
    if PAWN_MOVE.is_match(move_string) {
        let piece_to_move = colorize_piece(PAWN, board.state.side);
        let captures = PAWN_MOVE.captures(move_string).unwrap();
        let to = string_to_location(captures.name("to").unwrap().as_str());
        let piece_after_promotion = captures
            .name("piece_after_promotion")
            .map(|piece_after_promotion_capture| {
                get_piece_from_string(piece_after_promotion_capture.as_str(), board)
            })
            .unwrap_or(piece_to_move);
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
            && board.state.pieces[from as usize] == piece_to_move
            && (distance == 8) == (board.state.pieces[to as usize] == EMPTY_SQUARE)
            && !board.is_piece_pinned(from, to, king_location)
        {
            return Some(new_promotion(
                from as usize,
                to,
                piece_after_promotion,
                board,
            ));
        }
    } else if NOT_PAWN_MOVE.is_match(move_string) {
        let captures = NOT_PAWN_MOVE.captures(move_string).unwrap();
        let piece_to_move =
            get_piece_from_string(captures.name("piece_to_move").unwrap().as_str(), board);
        let to = string_to_location(captures.name("to").unwrap().as_str());
        let candidate_froms =
            board.get_pieces_of_type_defending_square_locations(to, piece_to_move);
        let unambiguity = captures
            .name("unambiguity")
            .map(|unambiguity| unambiguity.as_str());
        match get_unambiguous_from(&candidate_froms, unambiguity, piece_to_move, board) {
            Some(from) => {
                return Some(new_move(from as usize, to, board));
            }
            None => {}
        }
    }

    None
}
