use crate::board::{Board, BoardMove};

#[derive(Clone)]
pub struct OwnedBoard {
    pub(super) rows: u8,
    pub(super) columns: u8,
    pub(super) cells: Vec<u8>,
}

impl OwnedBoard {
    pub fn rows(&self) -> impl Iterator<Item = &[u8]> {
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

    fn empty_cell_pos(&self) -> (u8, u8) {
        let empty_cell_index = self
            .cells
            .iter()
            .position(|c| *c == 0)
            .expect("Cell vector does not contain empty cell");

        let row = empty_cell_index / self.columns as usize;
        let column = empty_cell_index % self.columns as usize;

        (row as u8, column as u8)
    }

    fn is_solved(&self) -> bool {
        // first check if the empty square is at the last position,
        // as in most cases that will not be the case,
        // thus eliminating the need for checking any other squares
        self.cells.last().copied().expect("cells cannot be empty") == 0
            // else we check all other squares and verify that they are in order
            && self
                .cells
                .iter()
                .copied()
                .zip(1..self.cells.len())
                .all(|(actual, expected)| actual == expected as u8)
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

    use crate::board::owned::OwnedBoard;
    use crate::board::*;

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
