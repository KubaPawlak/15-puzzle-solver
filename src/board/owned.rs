use crate::board::{Board, BoardMove};

#[derive(Clone)]
pub struct OwnedBoard {
    pub(super) rows: u8,
    pub(super) columns: u8,
    pub(super) cells: Vec<u8>,
}

impl OwnedBoard {
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
        match board_move {
            BoardMove::Up => self.empty_cell_pos().0 > 0,
            BoardMove::Down => self.empty_cell_pos().0 < self.rows - 1,
            BoardMove::Left => self.empty_cell_pos().1 > 0,
            BoardMove::Right => self.empty_cell_pos().1 < self.columns - 1,
        }
    }

    fn exec_move(&mut self, board_move: BoardMove) {
        assert!(self.can_move(board_move), "Board cannot execute this move");

        let (zero_row, zero_col) = self.empty_cell_pos();
        let (target_row, target_col) = match board_move {
            BoardMove::Up => (zero_row - 1, zero_col),
            BoardMove::Down => (zero_row + 1, zero_col),
            BoardMove::Left => (zero_row, zero_col - 1),
            BoardMove::Right => (zero_row, zero_col + 1),
        };

        let zero_index = self.flatten_index(zero_row, zero_col);
        let target_index = self.flatten_index(target_row, target_col);

        debug_assert_ne!(zero_index, target_index);

        let target_value = self.cells[target_index];
        self.cells[target_index] = 0;
        self.cells[zero_index] = target_value;
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
}
