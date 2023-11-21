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
            BoardMove::Up => write!(f, "U"),
            BoardMove::Down => write!(f, "D"),
            BoardMove::Left => write!(f, "L"),
            BoardMove::Right => write!(f, "R"),
        }
    }
}

pub fn generate_pairs_of_moves(previous_pair: Option<(BoardMove, BoardMove)>) -> Vec<(BoardMove, BoardMove)> {
    let moves = [BoardMove::Up, BoardMove::Down, BoardMove::Left, BoardMove::Right];
    let mut pairs = Vec::new();

    for &first_move in &moves {
        for &second_move in &moves {
            // Avoid obviously unsound moves
            if !is_opposite_move(first_move, second_move) {
                if let Some((_prev_first, prev_second)) = previous_pair {
                    //Avoid rewinding previous move
                    if !is_opposite_move(first_move, prev_second) {
                        pairs.push((first_move, second_move));
                    }
                }else { pairs.push((first_move, second_move)) }
            }
        }
    }
    pairs
}

fn is_opposite_move(move1: BoardMove, move2: BoardMove) -> bool {
    // Helper function to check if two moves are opposites (e.g., Up and Down)
    match (move1, move2) {
        (BoardMove::Up, BoardMove::Down) | (BoardMove::Down, BoardMove::Up) | (BoardMove::Left, BoardMove::Right) | (BoardMove::Right, BoardMove::Left) => true,
        _ => false,
    }
}

pub trait Board {
    /// Returns number of rows and columns
    fn dimensions(&self) -> (u8, u8);

    fn at(&self, row: u8, column: u8) -> u8;

    /// Returns the row and column index of the empty cell
    fn empty_cell_pos(&self) -> (u8, u8);

    fn is_solved(&self) -> bool;

    /// Checks if a given move can be performed on the board
    fn can_move(&self, board_move: BoardMove) -> bool;

    /// # Panics
    /// This function may panic if the move cannot be performed.
    /// To avoid it, check before if a move can be executed using [can_move](Board::can_move)
    fn exec_move(&mut self, board_move: BoardMove);
}
