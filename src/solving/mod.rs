use std::ops::Sub;

use parity::{calculate_parity, solved_board_parity, Parity};

use crate::board::Board;

mod movegen;
mod parity;

pub fn is_solvable(board: &impl Board) -> bool {
    fn manhattan_distance<T: Sub<Output = T> + Ord + Into<isize>>(p1: (T, T), p2: (T, T)) -> usize {
        abs_diff(p1.0, p2.0) + abs_diff(p1.1, p2.1)
    }

    fn abs_diff<T: Sub<Output = T> + Ord + Into<isize>>(x: T, y: T) -> usize {
        if x > y {
            (x - y).into() as usize
        } else {
            (y - x).into() as usize
        }
    }

    let (rows, columns) = board.dimensions();
    let mut cells = vec![];

    for row in 0..rows {
        for column in 0..columns {
            cells.push(board.at(row, column));
        }
    }

    let board_parity = calculate_parity(&cells);

    let solved_board_parity = solved_board_parity(board);

    let zero_manhattan_distance = {
        let final_empty_pos = (rows - 1, columns - 1);
        let current_empty_pos = board.empty_cell_pos();
        manhattan_distance(final_empty_pos, current_empty_pos)
    };

    board_parity + solved_board_parity == Parity::from(zero_manhattan_distance)
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
