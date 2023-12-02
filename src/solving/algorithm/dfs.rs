use std::collections::HashSet;

use crate::board::{Board, BoardMove, OwnedBoard};
use crate::solving::algorithm::Solver;
use crate::solving::is_solvable;
use crate::solving::movegen::{MoveGenerator, MoveSequence};

pub struct DFSSolver {
    visited: HashSet<OwnedBoard>,
    move_generator: MoveGenerator,
    current_path: Vec<BoardMove>,
    board: OwnedBoard,
}

impl DFSSolver {
    pub fn new(board: OwnedBoard, move_generator: MoveGenerator) -> Self {
        Self {
            board,
            visited: HashSet::new(),
            move_generator,
            current_path: vec![],
        }
    }

    fn perform_iteration(
        &mut self,
        current_depth: usize,
        max_depth: Option<usize>,
    ) -> Result<(), ()> {
        if self.board.is_solved() {
            return Ok(());
        }
        if self.visited.contains(&self.board) {
            return Err(());
        }
        self.visited.insert(self.board.clone());

        if let Some(max_depth) = max_depth {
            if current_depth >= max_depth {
                return Err(());
            }
        }

        for next_move in self
            .move_generator
            .generate_moves(&self.board, self.current_path.last().copied())
        {
            self.exec_move_sequence(&next_move);
            if self._call_recursive(current_depth + 1, max_depth).is_ok() {
                return Ok(());
            }
            self.undo_move_sequence(&next_move);
        }

        Err(())
    }

    fn _call_recursive(
        &mut self,
        current_depth: usize,
        max_depth: Option<usize>,
    ) -> Result<(), ()> {
        const STACK_RED_ZONE: usize = 64 * 1024;
        #[cfg(feature = "stack-expansion")]
        {
            // If we have less than `STACK_RED_ZONE` stack remaining, we allocate 4MB for a new stack
            if stacker::maybe_grow(STACK_RED_ZONE, 4 * 1024 * 1024, || {
                self.perform_iteration(current_depth + 1, max_depth)
            })
            .is_ok()
            {
                return Ok(());
            }
        }
        #[cfg(not(feature = "stack-expansion"))]
        {
            if let Some(remaining) = stacker::remaining_stack() {
                // If we have less than `STACK_RED_ZONE` stack remaining, we must backtrack to avoid stack overflow
                if remaining < STACK_RED_ZONE {
                    return Err(());
                }
            }
            if self.perform_iteration(current_depth + 1, max_depth).is_ok() {
                return Ok(());
            }
        }
        Err(())
    }

    fn exec_move_sequence(&mut self, move_sequence: &MoveSequence) {
        match *move_sequence {
            MoveSequence::Single(m) => {
                self.board.exec_move(m);
                self.current_path.push(m);
            }
            MoveSequence::Double(fst, snd) => {
                self.board.exec_move(fst);
                self.board.exec_move(snd);
                self.current_path.push(fst);
                self.current_path.push(snd);
            }
        }
    }

    fn undo_move_sequence(&mut self, move_sequence: &MoveSequence) {
        match move_sequence {
            MoveSequence::Single(m) => {
                self.board.exec_move(m.opposite());
                self.current_path.pop();
            }
            MoveSequence::Double(fst, snd) => {
                self.board.exec_move(snd.opposite());
                self.board.exec_move(fst.opposite());
                self.current_path.pop();
                self.current_path.pop();
            }
        }
    }
}

impl Solver for DFSSolver {
    fn solve(mut self: Box<Self>) -> Result<Vec<BoardMove>, ()> {
        if !is_solvable(&self.board) {
            return Err(());
        }

        self.perform_iteration(0, None)
            .expect("If board is solvable, DFS without depth limit should always find a solution");

        Ok(self.current_path)
    }
}

pub struct IncrementalDFSSolver {
    dfs_solver: DFSSolver,
}

impl IncrementalDFSSolver {
    pub fn new(board: OwnedBoard, move_generator: MoveGenerator) -> Self {
        Self {
            dfs_solver: DFSSolver::new(board, move_generator),
        }
    }
}

impl Solver for IncrementalDFSSolver {
    fn solve(mut self: Box<Self>) -> Result<Vec<BoardMove>, ()> {
        if !is_solvable(&self.dfs_solver.board) {
            return Err(());
        }

        let mut max_depth = 1;
        while self
            .dfs_solver
            .perform_iteration(0, Some(max_depth))
            .is_err()
        {
            max_depth += 1;
            self.dfs_solver.visited.clear();
        }

        Ok(self.dfs_solver.current_path)
    }
}

#[cfg(test)]
mod test {
    use crate::board::{Board, OwnedBoard};
    use crate::solving::parity::Parity;

    use super::*;

    #[test]
    fn does_backtrack_when_setting_was_already_found() {
        use crate::board::BoardMove::*;
        let board_str = r#"4 4
1  2  3  4
5  6  0  8
9  10 7  12
13 14 11 15
"#;
        let mut board: OwnedBoard = board_str.parse().unwrap();

        // odd parity is required so that only 1 move ahead is considered
        assert_eq!(
            crate::solving::parity::required_moves_parity(&board),
            Parity::Odd
        );

        let mut visited = HashSet::new();
        for m in [Up, Down, Left, Right] {
            board.exec_move(m);
            visited.insert(board.clone());
            board.exec_move(m.opposite());
        }

        let mut solver = DFSSolver {
            board,
            visited,
            move_generator: MoveGenerator::default(),
            current_path: vec![],
        };
        // at this point visited contains all the possible board positions that can be reached from the current state
        // therefore, it is expected that `perform_iteration` will return Err
        let result = solver.perform_iteration(0, None);

        assert!(result.is_err())
    }
}
