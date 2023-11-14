pub mod parsing;

#[repr(u8)]
pub enum BoardMove {
    Up,
    Down,
    Left,
    Right
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

#[derive(Clone)]
pub(crate) struct OwnedBoard {
    rows: u8,
    columns: u8,
    cells: Vec<u8>,
}

impl OwnedBoard {
    fn rows(&self) -> impl Iterator<Item=&[u8]> {
        self.cells.chunks(self.columns as usize)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [u8]> {
        self.cells.chunks_mut(self.columns as usize)
    }

    /// Convert 2D representation of cell coordinate to a single index in the underlying vec
    fn flatten_index(&self, row: u8, column: u8) -> usize {
        row as usize * self.rows as usize + column as usize
    }
}

impl Board for OwnedBoard {
    fn dimensions(&self) -> (u8, u8) {
        (self.rows, self.columns)
    }
    fn at(&self, row: u8, column: u8) -> u8 {
        self.cells[self.flatten_index(row, column)]
    }
    fn is_solved(&self) -> bool {
        self.cells[..self.cells.len() - 1]
            .windows(2)
            .all(|w| w[0] <= w[1])
            && self.cells[self.cells.len() - 1] == 0
    }

    fn can_move(&self, board_move: BoardMove) -> bool {
        todo!()
    }

    fn exec_move(&mut self, board_move: BoardMove) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::iter::once;

    use super::*;

    fn create_solved_board() -> OwnedBoard {
        OwnedBoard {
            rows: 4,
            columns: 4,
            cells: (1..=15).chain(once(0)).collect(),
        }
    }

    #[test]
    fn solved_board_shows_as_solved() {
        let solved_board = &create_solved_board();

        assert!(solved_board.is_solved());
    }

    #[test]
    fn rows_yields_correct_structure() {
        let solved_board = create_solved_board();
        let mut solved_rows = solved_board.rows();

        assert_eq!(solved_rows.next().unwrap(), &[1, 2, 3, 4]);
        assert_eq!(solved_rows.next().unwrap(), &[5, 6, 7, 8]);
        assert_eq!(solved_rows.next().unwrap(), &[9, 10, 11, 12]);
        assert_eq!(solved_rows.next().unwrap(), &[13, 14, 15, 0]);

        assert_eq!(
            solved_board.dimensions().0 as usize,
            solved_board.rows().count()
        );
    }
}
