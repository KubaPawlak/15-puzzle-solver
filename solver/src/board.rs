mod parsing;

pub(crate) struct Board {
    rows: u8,
    columns: u8,
    cells: Vec<u8>,
}

impl Board {
    /// Returns number of rows and columns
    pub fn dimensions(&self) -> (u8, u8) {
        (self.rows, self.columns)
    }

    pub fn rows(&self) -> Vec<&[u8]> {
        self.cells.chunks(self.columns as usize).collect()
    }

    pub fn rows_mut(&mut self) -> Vec<&mut [u8]> {
        self.cells.chunks_mut(self.columns as usize).collect()
    }

    pub fn at(&self, row: u8, column: u8) -> u8 {
        self.cells[self.flatten_index(row, column)]
    }

    pub fn is_solved(&self) -> bool {
        self.cells[..self.cells.len() - 1]
            .windows(2)
            .all(|w| w[0] <= w[1])
    }

    /// Convert 2D representation of cell coordinate to a single index in the underlying vec
    fn flatten_index(&self, row: u8, column: u8) -> usize {
        row as usize * self.rows as usize + column as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLVED_INPUT: &str = r"4 4
1  2  3  4
5  6  7  8
9 10 11 12
13 14 15 0
";
    #[test]
    fn solved_board_shows_as_solved() {
        let solved_board: Board = SOLVED_INPUT.parse().unwrap();

        assert!(solved_board.is_solved())
    }
}
