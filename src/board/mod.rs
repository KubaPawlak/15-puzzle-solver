use std::fmt::{Display, Formatter};

pub use owned::OwnedBoard;

mod owned;
mod parsing;
mod sub_board;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum BoardMove {
    Up,
    Down,
    Left,
    Right,
}

impl Display for BoardMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardMove::Up => write!(f,"U"),
            BoardMove::Down => write!(f,"D"),
            BoardMove::Left => write!(f,"L"),
            BoardMove::Right => write!(f,"R"),
        }
    }
}


pub trait Board {
    /// Returns number of rows and columns
    fn dimensions(&self) -> (u8, u8);
    fn at(&self, row: u8, column: u8) -> u8;
    fn is_solved(&self) -> bool;

    /// Checks if a given move can be performed on the board
    fn can_move(&self, board_move: BoardMove) -> bool;

    /// # Panics
    /// This function may panic if the move cannot be performed.
    /// To avoid it, check before if a move can be executed using [can_move](Board::can_move)
    fn exec_move(&mut self, board_move: BoardMove);
}
