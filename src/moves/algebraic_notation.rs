// This module doesn't have to be fast.
extern crate regex;

use crate::board::Board;
use crate::moves::constructors::{new_castling, new_en_passant, new_move, new_promotion};
use crate::moves::{
    get_captured_piece, get_from, get_move_type, get_moved_piece, get_promoted_piece, get_to, Move,
    MoveType, CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE, EN_PASSANT, NORMAL_MOVE,
};
use crate::pieces::color::{colorize_piece, uncolorize_piece};
use crate::pieces::pawn;
use crate::pieces::pawn::PAWN_STEPS;
use crate::pieces::{ColorizedPiece, Piece, BISHOP, EMPTY_SQUARE, KING, KNIGHT, PAWN, QUEEN, ROOK};

use regex::Regex;

fn file_to_char(file: i8) -> char {
    (file + 97) as u8 as char
}

fn rank_to_char(rank: i8) -> char {
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

fn location_to_string(location: i8) -> String {
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
    let attackers_locations: Vec<i8> = board
        .get_pieces_of_type_defending_square_locations(to as i8, moved_piece)
        .iter()
        .filter(|&&location| location != from)
        .map(|&location| location)
        .collect();

    if attackers_locations.is_empty() {
        return;
    }

    let from_file = from & 7;
    let from_rank = from >> 3;

    if attackers_locations
        .iter()
        .filter(|&&location| location & 7 == from_file)
        .count()
        == 0
    {
        move_string_part.push(file_to_char(from_file));
    } else if attackers_locations
        .iter()
        .filter(|&&location| location >> 3 == from_rank)
        .count()
        == 0
    {
        move_string_part.push(rank_to_char(from_rank));
    } else {
        move_string_part.push_str(location_to_string(from).as_str());
    }
}

pub fn to_algebraic_notation(half_move: Move, board: &mut Board) -> String {
    match get_move_type(half_move) {
        CASTLING_KINGS_SIDE => String::from("O-O"),
        CASTLING_QUEENS_SIDE => String::from("O-O-O"),
        EN_PASSANT | NORMAL_MOVE => {
            let mut result = String::new();
            let moved_piece = get_moved_piece(half_move);
            let promoted_piece = get_promoted_piece(half_move);
            let uncolorized_moved_piece = uncolorize_piece(moved_piece);
            let is_capture = get_captured_piece(half_move) != EMPTY_SQUARE;
            if uncolorized_moved_piece != PAWN {
                result.push(piece_to_char(uncolorized_moved_piece));
                remove_ambiguities(half_move, &mut result, board);
            } else if is_capture {
                let from_file = (get_from(half_move) & 7) as i8;
                result.push(file_to_char(from_file));
            }
            // Rank or file will be here.
            if is_capture {
                result.push('x');
            }

            let to = get_to(half_move) as i8;
            result.push_str(location_to_string(to).as_str());

            if uncolorized_moved_piece == PAWN && moved_piece != promoted_piece {
                result.push('=');
                result.push(piece_to_char(uncolorize_piece(promoted_piece)));
            }
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

fn parse_pawn_move(captures: regex::Captures<'_>, board: &mut Board) -> Option<Move> {
    let color = board.state.side as usize;
    let piece_to_move = colorize_piece(PAWN, board.state.side);
    let to = string_to_location(captures.name("to").unwrap().as_str());
    let piece_after_promotion = captures
        .name("piece_after_promotion")
        .map(|piece_after_promotion_capture| {
            get_piece_from_string(piece_after_promotion_capture.as_str(), board)
        })
        .unwrap_or(piece_to_move);

    let pawn_step = PAWN_STEPS[color][1];
    let from = match captures.name("file") {
        Some(file_capture) => {
            let file = char_to_file(first_char(file_capture.as_str()));
            to - (to & 7) - pawn_step + file
        }
        None => {
            let from = to - pawn_step;
            if board.state.pieces[from as usize] == piece_to_move {
                from
            } else {
                from - pawn_step
            }
        }
    };

    if pawn::is_legal_en_passant(from, to, piece_to_move, piece_after_promotion, board) {
        return Some(new_en_passant(from as usize, to, board.state.side, board));
    }

    match pawn::is_move_legal(from, to, piece_to_move, piece_after_promotion, board) {
        true => Some(new_promotion(
            from as usize,
            to,
            piece_after_promotion,
            board,
        )),
        false => None,
    }
}

fn parse_not_pawn_move(captures: regex::Captures<'_>, board: &mut Board) -> Option<Move> {
    let piece_to_move =
        get_piece_from_string(captures.name("piece_to_move").unwrap().as_str(), board);
    let to = string_to_location(captures.name("to").unwrap().as_str());
    let candidate_froms = board.get_pieces_of_type_defending_square_locations(to, piece_to_move);
    let unambiguity = captures
        .name("unambiguity")
        .map(|unambiguity| unambiguity.as_str());
    get_unambiguous_from(&candidate_froms, unambiguity, piece_to_move, board)
        .map(|from| new_move(from as usize, to, board))
}

fn parse_castling(castling_type: MoveType, board: &Board) -> Option<Move> {
    let king_location = board.state.king_positions[board.state.side as usize] as usize;
    let king = board.state.pieces[king_location];
    Some(new_castling(
        castling_type,
        king_location,
        king,
        board.state.side,
    ))
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

    return if PAWN_MOVE.is_match(move_string) {
        parse_pawn_move(PAWN_MOVE.captures(move_string).unwrap(), board)
    } else if NOT_PAWN_MOVE.is_match(move_string) {
        parse_not_pawn_move(NOT_PAWN_MOVE.captures(move_string).unwrap(), board)
    } else if move_string == "O-O" && board.is_castling_kings_side_legal() {
        parse_castling(CASTLING_KINGS_SIDE, board)
    } else if move_string == "O-O-O" && board.is_castling_queens_side_legal() {
        parse_castling(CASTLING_QUEENS_SIDE, board)
    } else {
        None
    };
}
