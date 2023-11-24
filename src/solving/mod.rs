use parity::{permuation_parity, required_moves_parity, solved_board_parity};

use crate::board::Board;

pub mod algorithm;
mod movegen;
mod parity;

fn is_solvable(board: &impl Board) -> bool {
    let (rows, columns) = board.dimensions();
    let mut cells = vec![];

    for row in 0..rows {
        for column in 0..columns {
            cells.push(board.at(row, column));
        }
    }

    let board_parity = permuation_parity(&cells);

    let solved_board_parity = solved_board_parity(board);

    board_parity + required_moves_parity(board) == solved_board_parity
}

#[cfg(test)]
mod test {
    use crate::board::OwnedBoard;
    use crate::solving::is_solvable;

    #[test]
    fn solvable_board_shows_as_solvable() {
        let solvable_input = r"4 4
1  2  3  4
5  6  7  8
9 10 11 12
13 14 0 15
";
        let solvable_board: OwnedBoard = solvable_input.parse().unwrap();
        assert!(is_solvable(&solvable_board));
    }

    #[test]
    fn unsolvable_board_shows_as_not_solvable() {
        let unsolvable_input = r"4 4
1  2  3  4
5  6  7  8
9 10 11 12
13 15 14 0
";
        let unsolvable_board: OwnedBoard = unsolvable_input.parse().unwrap();
        assert!(!is_solvable(&unsolvable_board));
    }
}
