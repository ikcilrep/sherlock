use crate::pieces::color::Color;

pub enum GameResult {
    Draw,
    Win(Color),
    StillInProgress,
}

impl GameResult {
    pub fn get_points(&self, side: Color) -> i32 {
        match *self {
            GameResult::Draw => 1,
            GameResult::Win(color) => return if color == side { 2 } else { 0 },
            GameResult::StillInProgress => panic!("Can't get points of unfinished game."),
        }
    }
    pub fn is_in_progress(&self) -> bool {
        match *self {
            GameResult::StillInProgress => true,
            _ => false,
        }
    }
}
