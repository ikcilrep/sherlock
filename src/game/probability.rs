extern crate rand;
use crate::board::Board;
use crate::game::play_random_game;
use crate::moves::Move;
use crate::pieces::color::Color;
use rand::rngs::ThreadRng;

impl Board {
    pub fn get_position_score(&self, color: Color, repetitions: i32, rng: &mut ThreadRng) -> i32 {
        let mut result = 0;
        for _ in 0..repetitions {
            let mut self_clone = self.clone();
            result += play_random_game(&mut self_clone, rng).get_points(color);
        }
        result
    }

    pub fn get_best_move(&mut self, repetitions: i32, rng: &mut ThreadRng) -> Move {
        let moves = self.generate_all_legal_moves();
        let mut best_score = -1;
        let mut best_move = *moves.iter().next().unwrap();
        for half_move in moves {
            let mut self_clone = self.clone();
            self_clone.make_move(half_move);
            let score = self_clone.get_position_score(self.state.side, repetitions, rng);
            if score > best_score {
                best_score = score;
                best_move = half_move;
            }
        }
        best_move
    }
}
