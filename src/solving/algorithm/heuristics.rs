use std::cmp::{max, min};

use crate::board::Board;

pub trait Heuristic {
    /// Calculates the heuristic for a given board setting.
    /// The heuristic is the lower bound on the required number of moves
    fn evaluate(&self, board: &dyn Board) -> u64;
}

#[derive(Default)]
pub struct ManhattanDistance;

fn manhattan_distance((r1, c1): (u8, u8), (r2, c2): (u8, u8)) -> u64 {
    let row_distance = max(r1, r2) - min(r1, r2);
    let column_distance = max(c1, c2) - min(c1, c2);
    row_distance as u64 + column_distance as u64
}

impl Heuristic for ManhattanDistance {
    fn evaluate(&self, board: &dyn Board) -> u64 {
        let (rows, columns) = board.dimensions();
        let target_position = |cell: u8| (cell / rows, cell % columns);

        let mut total_distance = 0;

        for row in 0..rows {
            for column in 0..columns {
                let value = board.at(row, column);
                let distance = manhattan_distance((row, column), target_position(value));
                total_distance += distance;
            }
        }

        total_distance
    }
}

#[derive(Default)]
pub struct LinearConflict {
    manhattan_distance: ManhattanDistance,
}

impl Heuristic for LinearConflict {
    fn evaluate(&self, board: &dyn Board) -> u64 {
        let (rows, columns) = board.dimensions();
        let mut conflicts = 0;

        // calculate row conflicts
        for row in 0..rows {
            for first_column in 0..(columns - 1) {
                for second_column in (first_column + 1)..columns {
                    if board.at(row, first_column) > board.at(row, second_column) {
                        conflicts += 1;
                    }
                }
            }
        }

        // calculate column conflicts
        for column in 0..columns {
            for first_row in 0..(rows - 1) {
                for second_row in (first_row + 1)..rows {
                    if board.at(first_row, column) > board.at(second_row, column) {
                        conflicts += 1;
                    }
                }
            }
        }

        self.manhattan_distance.evaluate(board) + conflicts * 2 // for each conflict we need at least 2 moves
    }
}

/// Implementation of heuristic developed by Ken'ichiro Takahashi
/// Description of the heuristic can be found at https://computerpuzzle.net/puzzle/15puzzle/index.html
#[derive(Default)]
pub struct InversionDistance {
    cache: std::cell::RefCell<Option<InversionDistanceCache>>,
}

struct InversionDistanceCache {
    rows: u8,
    columns: u8,
    row_first_order: Box<[u8]>,
    column_first_order: Box<[u8]>,
}

impl InversionDistanceCache {
    pub fn new(board: &dyn Board) -> Self {
        let (rows, columns) = board.dimensions();
        let rows_first_order: Vec<_> = (1..(rows * columns)).chain(std::iter::once(0)).collect();
        let mut column_first_order = vec![];
        for c in 0..columns {
            for r in 0..rows {
                column_first_order.push(r * rows + c + 1);
            }
        }

        // last cell should be 0
        column_first_order[(rows * columns - 1) as usize] = 0;

        Self {
            rows,
            columns,
            row_first_order: rows_first_order.into_boxed_slice(),
            column_first_order: column_first_order.into_boxed_slice(),
        }
    }
}

impl InversionDistance {
    fn number_of_inversions(order: &[u8], expected_order: &[u8]) -> u64 {
        assert_eq!(order.len(), expected_order.len());

        let mut num_inversions = 0;

        for i in 0..(order.len() - 1) {
            for j in i..order.len() {
                let first = order[i];
                let second = order[j];

                if first == 0 || second == 0 {
                    continue; // empty cell does not contribute to inversions
                }

                // check if they are in invalid inversion
                if expected_order
                    .iter()
                    .position(|&x| x == first)
                    .expect("Element has to be in expected order, since it comes from a board")
                    > expected_order
                        .iter()
                        .position(|&x| x == second)
                        .expect("Element has to be in expected order, since it comes from a board")
                {
                    num_inversions += 1;
                }
            }
        }

        num_inversions
    }
}

impl Heuristic for InversionDistance {
    fn evaluate(&self, board: &dyn Board) -> u64 {
        let dimensions = board.dimensions();

        // instantiate cache if empty or has wrong dimensions
        let mut cache = self.cache.try_borrow_mut().unwrap();
        if !matches!(*cache, Some(InversionDistanceCache{rows, columns, ..}) if (rows, columns) == dimensions )
        {
            // if cache is empty or invalid size
            *cache = Some(InversionDistanceCache::new(board));
        }
        let cache = cache.as_ref().expect("Cache was just instantiated");

        let (rows, columns) = dimensions;
        let mut row_first_order = vec![];
        for row in 0..rows {
            for column in 0..columns {
                row_first_order.push(board.at(row, column));
            }
        }
        let mut column_first_order = vec![];
        for column in 0..columns {
            for row in 0..rows {
                column_first_order.push(board.at(row, column));
            }
        }

        let mut row_inversions =
            Self::number_of_inversions(&row_first_order, &cache.row_first_order);
        let mut column_inversions =
            Self::number_of_inversions(&column_first_order, &cache.column_first_order);

        let mut vertical = 0;
        let mut divisor = columns as u64 - 1;
        while divisor > 0 {
            vertical += row_inversions / divisor;
            row_inversions %= divisor;
            divisor = divisor.saturating_sub(2);
        }

        let mut horizontal = 0;
        let mut divisor = rows as u64 - 1;
        while divisor > 0 {
            horizontal += column_inversions / divisor;
            column_inversions %= divisor;
            divisor = divisor.saturating_sub(2);
        }

        vertical + horizontal
    }
}

#[cfg(test)]
mod tests {
    use crate::board::OwnedBoard;
    use crate::solving::algorithm::dfs::IncrementalDFSSolver;
    use crate::solving::algorithm::Solver;
    use crate::solving::movegen::MoveGenerator;

    use super::*;

    fn create_board() -> OwnedBoard {
        let board_str = r#"4 4
2  7  3  4
1  0  10 8
5  6  12 15
9 13  14 11
"#;
        board_str.parse::<OwnedBoard>().unwrap()
    }

    fn heuristic_calculates_lower_bound_on_required_moves(heuristic: &dyn Heuristic) {
        let mut board = create_board();

        let solution = {
            let solver = IncrementalDFSSolver::new(board.clone(), MoveGenerator::default());
            solver.solve().expect("Test board must be solvable")
        };

        for i in 0..solution.len() {
            let remaining_moves = (solution.len() - i) as u64;
            let heuristic = heuristic.evaluate(&board);
            assert!(heuristic >= remaining_moves);
            board.exec_move(solution[i]);
        }
    }

    #[test]
    fn manhattan_distance_is_admissible() {
        let heuristic = ManhattanDistance;
        heuristic_calculates_lower_bound_on_required_moves(&heuristic);
    }

    #[test]
    fn linear_conflict_is_admissible() {
        let heuristic = LinearConflict::default();
        heuristic_calculates_lower_bound_on_required_moves(&heuristic);
    }

    #[test]
    fn inversion_distance_is_admissible() {
        let heuristic = InversionDistance::default();
        heuristic_calculates_lower_bound_on_required_moves(&heuristic);
    }
}
