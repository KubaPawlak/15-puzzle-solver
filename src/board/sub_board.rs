use crate::board::{Board, BoardMove};

pub struct SubBoard<'a> {
    // 'a is a lifetime parameter. Bez niego krzyczy.
    original_board: &'a mut dyn Board,
    rows: u8,
    columns: u8,
    translation_offset: u8,
}
    impl<'a> SubBoard<'a> {
        pub fn new_sub_board(original_board: &'a mut dyn Board, translation_offset: u8, rows: u8, columns: u8) -> Self {
            /*
            if ()
            {
                panic!("Sub board must contain the empty cell.");
            }
            */
            Self {
                original_board,
                translation_offset,
                rows,
                columns
            }
        }
    }

impl Board for SubBoard{
    fn dimensions(&self) -> (u8, u8) {
        (self.rows, self.columns)
    }

    fn at(&self, row: u8, column: u8) -> u8 {
        todo!()
    }

    fn empty_cell_pos(&self) -> (u8, u8) {
        todo!()
    }

    fn is_solved(&self) -> bool {
        todo!()
    }

    fn can_move(&self, board_move: BoardMove) -> bool {
        todo!()
    }

    fn exec_move(&mut self, board_move: BoardMove) {
        todo!()
    }
}