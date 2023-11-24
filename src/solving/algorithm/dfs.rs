use std::collections::HashSet;

use crate::board::{Board, BoardMove, OwnedBoard};
use crate::solving::is_solvable;
use crate::solving::movegen::{next_moves, MoveSequence};

pub struct DFSSolver {
    visited: HashSet<OwnedBoard>,
    current_path: Vec<BoardMove>,
    board: OwnedBoard,
}

impl DFSSolver {
    pub fn new(board: OwnedBoard) -> Self {
        Self {
            board,
            visited: HashSet::new(),
            current_path: vec![],
        }
    }

    pub fn solve(mut self) -> Result<Vec<BoardMove>, ()> {
        if !is_solvable(&self.board) {
            return Err(());
        }

        self.perform_iteration(0, None)
            .expect("If board is solvable, DFS without depth limit should always find a solution");

        Ok(self.current_path)
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

        for next_move in next_moves(&self.board, self.current_path.last().copied()) {
            self.exec_move_sequence(&next_move);
            if self.perform_iteration(current_depth + 1, max_depth).is_ok() {
                return Ok(());
            }
            self.undo_move_sequence(&next_move);
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
            current_path: vec![],
        };
        // at this point visited contains all the possible board positions that can be reached from the current state
        // therefore, it is expected that `perform_iteration` will return Err
        let result = solver.perform_iteration(0, None);

        assert!(result.is_err())
    }
}
