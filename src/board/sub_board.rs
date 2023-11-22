use crate::board::{Board, BoardMove};

pub struct SubBoard<'a> {
    // 'a is a lifetime parameter. Bez niego krzyczy.
    original_board: &'a mut dyn Board,
    starting_row: u8,
    starting_column: u8
}
impl<'a> SubBoard<'a> {
    pub fn new_sub_board(
        original_board: &'a mut dyn Board,
        starting_row: u8,
        starting_column: u8
    ) -> Self {
        let empty_pos = original_board.empty_cell_pos();

        assert!(
            Self::is_position_inside_sub_board(empty_pos, starting_row, starting_column),
            "Sub board must contain the empty cell."
        );

        Self {
            original_board,
            starting_row,
            starting_column
        }
    }
    fn is_position_inside_sub_board(pos: (u8, u8), starting_row: u8, starting_column: u8) -> bool {
        let (row, col) = pos;
        // Check if the position is greater than or equal to the starting row and column
        row >= starting_row && col >= starting_column
    }

}

impl<'a> Board for SubBoard<'a> {
    fn dimensions(&self) -> (u8, u8) {
        let (original_rows, original_columns) = self.original_board.dimensions();
        let subboard_rows = original_rows - self.starting_row;
        let subboard_columns = original_columns - self.starting_column;
        (subboard_rows, subboard_columns)
    }

    fn at(&self, row: u8, column: u8) -> u8 {
        todo!()
    }

    fn empty_cell_pos(&self) -> (u8, u8) {
        let (original_rows, original_columns) = self.original_board.dimensions();
        let (empty_row, empty_col) = self.original_board.empty_cell_pos();

        // Calculate the translated position based on starting row and column
        let translated_empty_row = empty_row - self.starting_row;
        let translated_empty_col = empty_col - self.starting_column;

        // Check if the translated position is within the subboard
        if translated_empty_row >= 0 && translated_empty_row < original_rows - self.starting_row
            && translated_empty_col >= 0 && translated_empty_col < original_columns - self.starting_column
        {
            (translated_empty_row as u8, translated_empty_col as u8)
        } else {
            panic!("Empty cell is not within the subboard.");
        }
    }

    fn is_solved(&self) -> bool {
        let (subboard_rows, subboard_columns) = self.dimensions();

        // Check if the empty cell is at the last position
        let empty_pos = self.original_board.empty_cell_pos();
        if empty_pos == (self.starting_row + subboard_rows - 1, self.starting_column + subboard_columns - 1) {
            // Check if the remaining cells are in order
            let mut expected = 1;
            for row in self.starting_row..self.starting_row + subboard_rows {
                for col in self.starting_column..self.starting_column + subboard_columns {
                    let cell_value = self.at(row, col);
                    if cell_value != expected {
                        return false;
                    }
                    expected += 1;
                }
            }
            true
        } else {
            false
        }
    }

    fn can_move(&self, board_move: BoardMove) -> bool {
        todo!()
    }

    fn exec_move(&mut self, board_move: BoardMove) {
        todo!()
    }
}
