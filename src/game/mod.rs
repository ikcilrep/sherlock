pub mod probability;
pub mod result;

use rand::rngs::ThreadRng;

use crate::board::Board;
use crate::game::result::GameResult;

pub fn play_random_game(board: &mut Board, rng: &mut ThreadRng) -> GameResult {
    loop {
        let king_attackers_locations =
            board.get_attackers_of_king_square_locations(board.state.side);
        let state = board.get_game_state(&king_attackers_locations);
        match state {
            GameResult::StillInProgress => {}
            _ => return state,
        }

        let half_move = if king_attackers_locations.is_empty() {
            board.generate_random_legal_move(rng)
        } else {
            board
                .generate_random_out_of_check_move(&king_attackers_locations, rng)
                .unwrap()
        };

        board.make_move(half_move);
    }
}

pub fn play_best_game(board: &mut Board, repetitions: i32, rng: &mut ThreadRng) -> GameResult {
    loop {
        let king_attackers_locations =
            board.get_attackers_of_king_square_locations(board.state.side);
        let state = board.get_game_state(&king_attackers_locations);
        match state {
            GameResult::StillInProgress => {}
            _ => return state,
        }

        let half_move = board.get_best_move(repetitions, rng);

        board.make_move(half_move);
    }
}
