pub type Color = i8;
pub const WHITE: Color = 0;
pub const BLACK: Color = 1;

#[macro_export]
macro_rules! color {
    ($piece: expr) => {
        ($piece) & 1
    };
}
