use crate::board::Board;
use crate::pieces::color::{colorize_piece, get_piece_color, Color};
use crate::pieces::{king, knight};
use crate::pieces::{ColorizedPiece, BISHOP, EMPTY_SQUARE, KING, KNIGHT, PAWN, QUEEN, ROOK};

const INVERSED_PAWN_CAPTURES: [[i8; 2]; 2] = [[-7, -9], [9, 7]];

impl Board {
    #[inline]
    fn is_square_not_occupied_by_color(self: &Board, square: usize, color: Color) -> bool {
        self.pieces[square] == EMPTY_SQUARE || get_piece_color(self.pieces[square]) != color
    }

    fn is_square_attacked_by_slider(
        self: &Board,
        square: i8,
        possible_attacker: ColorizedPiece,
        attacked_color: Color,
        increment: i8,
        predicate: fn(i8, i8) -> bool,
    ) -> bool {
        let colorized_queen = colorize_piece(QUEEN, !attacked_color);
        let mut attacker_square = square + increment;
        let mut square_file = square & 7;
        while predicate(attacker_square, square_file)
            && self.is_square_not_occupied_by_color(attacker_square as usize, attacked_color)
        {
            if self.pieces[attacker_square as usize] == possible_attacker
                || self.pieces[attacker_square as usize] == colorized_queen
            {
                return true;
            }
            attacker_square += increment;
        }
        false
    }

    fn is_square_attacked_on_straight_line(
        self: &Board,
        square: i8,
        attacked_color: Color,
    ) -> bool {
        let colorized_rook = colorize_piece(ROOK, !attacked_color);
        self.is_square_attacked_by_slider(
            square,
            colorized_rook,
            attacked_color,
            8,
            |attacker_square, _| attacker_square < 64,
        ) || self.is_square_attacked_by_slider(
            square,
            colorized_rook,
            attacked_color,
            -8,
            |attacker_square, _| attacker_square > 0,
        ) || self.is_square_attacked_by_slider(
            square,
            colorized_rook,
            attacked_color,
            1,
            |attacker_square, _| attacker_square & 7 != 0,
        ) || self.is_square_attacked_by_slider(
            square,
            colorized_rook,
            attacked_color,
            -1,
            |attacker_square, _| attacker_square & 7 != 7,
        )
    }

    #[inline]
    fn is_square_on_board(self: &Board, square: i8) -> bool {
        square >= 0 && square < 64
    }

    fn is_square_attacked_by_pawn(self: &Board, square: i8, attacked_color: Color) -> bool {
        let colorized_pawn = colorize_piece(PAWN, !attacked_color);
        let square_file = square & 7;
        let attacker_square1 = square + INVERSED_PAWN_CAPTURES[attacked_color as usize][0];
        let attacker_square2 = square + INVERSED_PAWN_CAPTURES[attacked_color as usize][1];
        (self.is_square_on_board(attacker_square1)
            && attacker_square1 & 7 > square_file
            && self.pieces[attacker_square1 as usize] == colorized_pawn)
            || (self.is_square_on_board(attacker_square2)
                && attacker_square2 & 7 < square_file
                && self.pieces[attacker_square2 as usize] == colorized_pawn)
    }

    fn is_square_attacked_on_diagonal(self: &Board, square: i8, attacked_color: Color) -> bool {
        let colorized_bishop = colorize_piece(BISHOP, !attacked_color);
        self.is_square_attacked_by_pawn(square, attacked_color)
            || self.is_square_attacked_by_slider(
                square,
                colorized_bishop,
                attacked_color,
                9,
                |attacker_square, square_file| {
                    attacker_square < 64 && attacker_square & 7 > square_file
                },
            )
            || self.is_square_attacked_by_slider(
                square,
                colorized_bishop,
                attacked_color,
                -9,
                |attacker_square, square_file| {
                    attacker_square > 0 && attacker_square & 7 < square_file
                },
            )
            || self.is_square_attacked_by_slider(
                square,
                colorized_bishop,
                attacked_color,
                7,
                |attacker_square, square_file| {
                    attacker_square < 64 && attacker_square & 7 < square_file
                },
            )
            || self.is_square_attacked_by_slider(
                square,
                colorized_bishop,
                attacked_color,
                -7,
                |attacker_square, square_file| {
                    attacker_square > 0 && attacker_square & 7 > square_file
                },
            )
    }

    fn is_square_attacked_by_knight(self: &Board, square: i8, attacked_color: Color) -> bool {
        let move_pseudo_legality_validators = [
            knight::is_move_northeast_pseudo_legal,
            knight::is_move_northwest_pseudo_legal,
            knight::is_move_southeast_pseudo_legal,
            knight::is_move_southwest_pseudo_legal,
            knight::is_move_northeast_pseudo_legal,
            knight::is_move_northwest_pseudo_legal,
            knight::is_move_southeast_pseudo_legal,
            knight::is_move_southwest_pseudo_legal,
        ];
        let moves_to = [
            square + 17,
            square + 15,
            square - 17,
            square - 15,
            square + 10,
            square + 6,
            square - 10,
            square - 6,
        ];
        let colorized_knight = colorize_piece(KNIGHT, !attacked_color);
        self.is_square_attacked_by_piece(
            square,
            colorized_knight,
            moves_to,
            move_pseudo_legality_validators,
            attacked_color,
        )
    }

    fn is_square_attacked_by_king(self: &Board, square: i8, attacked_color: Color) -> bool {
        let moves_to = king::get_moves_to(square as usize);
        let colorized_king = colorize_piece(KING, !attacked_color);
        self.is_square_attacked_by_piece(
            square,
            colorized_king,
            moves_to,
            king::MOVE_PSEUDO_LEGALITY_VALIDATORS,
            attacked_color,
        )
    }

    fn is_square_attacked_by_piece(
        self: &Board,
        square: i8,
        piece: ColorizedPiece,
        moves_to: [i8; 8],
        move_pseudo_legality_validators: [fn(i8, i8, &Board, Color) -> bool; 8],
        attacked_color: Color,
    ) -> bool {
        let square_file = square & 7;

        move_pseudo_legality_validators
            .iter()
            .zip(moves_to.iter())
            .any(|(is_move_pseudo_legal, attacker_square)| {
                is_move_pseudo_legal(square_file, *attacker_square, self, !attacked_color)
                    && self.pieces[*attacker_square as usize] == piece
            })
    }

    // Probably, to be optimized.
    pub fn is_square_attacked(self: &Board, square: i8, attacked_color: Color) -> bool {
        self.is_square_attacked_by_king(square, attacked_color)
            || self.is_square_attacked_by_knight(square, attacked_color)
            || self.is_square_attacked_on_straight_line(square, attacked_color)
            || self.is_square_attacked_on_diagonal(square, attacked_color)
    }
}
