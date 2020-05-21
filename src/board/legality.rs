use crate::board::Board;
use crate::moves::Move;
use crate::moves::{get_from, get_to};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::{bishop, king, knight, pawn, queen, rook, EMPTY_SQUARE};

enum GameState {
    Draw,
    Win(Color),
    StillInProgress,
}

pub const MOVE_AVAILABILITY_VALIDATORS: [fn(usize, &mut Board) -> bool; 6] = [
    pawn::can_be_moved,
    rook::can_be_moved,
    knight::can_be_moved,
    bishop::can_be_moved,
    queen::can_be_moved,
    king::can_be_moved,
    |_, _| false,
];

impl Board {
    fn is_piece_pinned(self: &mut Board, from: i8, to: i8, protected_piece_location: i8) -> bool {
        let protected_piece_file = protected_piece_location & 7;
        let protected_piece_rank = protected_piece_location >> 3;
        let from_file = from & 7;
        let from_rank = from >> 3;
        let to_file = to & 7;
        let to_rank = to >> 3;
        let difference = protected_piece_location - from;
        // I know how hard to read it is, but it's fast.
        (difference > 0
            && ((from_file == protected_piece_file
                && to_file != from_file
                && self
                    .is_square_attacked_from_south_straight_line_by_slider(from, self.state.side))
                || (from_rank == protected_piece_rank
                    && to_rank != from_rank
                    && self.is_square_attacked_from_west_straight_line_by_slider(
                        from,
                        self.state.side,
                    ))
                || (difference % 7 == 0
                    && self.is_square_attacked_from_northwest_diagonal_by_slider(
                        from,
                        self.state.side,
                    ))
                || (difference % 9 == 0
                    && self.is_square_attacked_from_northeast_diagonal_by_slider(
                        from,
                        self.state.side,
                    ))))
            || (difference <= 0
                && ((from_file == protected_piece_file
                    && to_file != from_file
                    && self.is_square_attacked_from_north_straight_line_by_slider(
                        from,
                        self.state.side,
                    ))
                    || (from_rank == protected_piece_rank
                        && to_rank != from_rank
                        && self.is_square_attacked_from_east_straight_line_by_slider(
                            from,
                            self.state.side,
                        )))
                || (difference % 7 == 0
                    && self.is_square_attacked_from_southeast_diagonal_by_slider(
                        from,
                        self.state.side,
                    ))
                || (difference % 9 == 0
                    && self.is_square_attacked_from_southwest_diagonal_by_slider(
                        from,
                        self.state.side,
                    )))
    }

    pub fn is_move_legal(self: &mut Board, half_move: Move) -> bool {
        let king_location = self.state.king_positions[self.state.side as usize];
        let from = get_from(half_move) as i8;

        let to = get_to(half_move) as i8;
        if from == king_location {
            let king = self.pieces[king_location as usize];
            self.pieces[king_location as usize] = EMPTY_SQUARE;

            let result = self.is_square_attacked(to, self.state.side);
            self.pieces[king_location as usize] = king;
            return result;
        }
        return !self.is_piece_pinned(from, to, king_location);
    }

    fn is_king_checked(self: &Board, color: Color) -> bool {
        self.is_square_attacked(self.state.king_positions[color as usize], color)
    }

    fn can_any_piece_be_moved(self: &mut Board) -> bool {
        self.pieces
            .iter()
            .filter(|piece| get_piece_color(**piece) == self.state.side)
            .enumerate()
            .any(|(from, piece)| MOVE_AVAILABILITY_VALIDATORS[*piece as usize](from, self))
    }

    fn get_game_result(self: &mut Board) -> GameState {
        // Temporary solution !self.can_any_piece_be_moved will be replaced with more customized function if king is checked.
        // Threefold repetition draw will be implemented in future.
        return if !self.can_any_piece_be_moved() {
            if self.is_king_checked(self.state.side) {
                GameState::Win(!self.state.side)
            } else {
                GameState::Draw
            }
        } else if self.state.fifty_moves == 100 {
            GameState::Draw
        } else {
            GameState::StillInProgress
        };
    }
}
