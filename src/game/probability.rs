extern crate rand;
use crate::board::Board;
use crate::game::play_random_game;
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
}
