use crate::pieces::color::Color;

pub enum GameResult {
    Draw,
    Win(Color),
    StillInProgress,
}
