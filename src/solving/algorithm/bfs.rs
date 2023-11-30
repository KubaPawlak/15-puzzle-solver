use std::collections::{HashSet, VecDeque};

use crate::board::{Board, BoardMove, OwnedBoard};
use crate::solving::algorithm::Solver;
use crate::solving::is_solvable;
use crate::solving::movegen::{MoveGenerator, MoveSequence};

pub struct BFSSolver {
    visited: HashSet<OwnedBoard>,
    move_generator: MoveGenerator,
    current_path: Vec<BoardMove>,
    board: OwnedBoard,
}

impl BFSSolver {
    pub fn new(board: OwnedBoard, move_generator: MoveGenerator) -> Self {
        Self {
            board,
            visited: HashSet::new(),
            move_generator,
            current_path: vec![],
        }
    }

    fn perform_iteration(&mut self) -> Result<(), ()> {
        let mut queue = VecDeque::new(); // Use a queue for BFS
        queue.push_back((self.board.clone(), self.current_path.clone()));

        while let Some((current_board, current_path)) = queue.pop_front() {
            if current_board.is_solved() {
                self.board = current_board;
                self.current_path = current_path;
                return Ok(());
            }

            if self.visited.contains(&current_board) {
                continue;
            }

            self.visited.insert(current_board.clone());

            for next_move in self.move_generator.generate_moves(&current_board, None) {
                let mut new_board = current_board.clone();
                let mut new_path = current_path.clone();
                self.exec_move_sequence(&mut new_board, &mut new_path, &next_move);
                queue.push_back((new_board, new_path));
            }
        }

        Err(())
    }

    fn exec_move_sequence(
        &mut self,
        board: &mut OwnedBoard,
        path: &mut Vec<BoardMove>,
        move_sequence: &MoveSequence,
    ) {
        match *move_sequence {
            MoveSequence::Single(m) => {
                board.exec_move(m);
                path.push(m);
            }
            MoveSequence::Double(fst, snd) => {
                board.exec_move(fst);
                board.exec_move(snd);
                path.push(fst);
                path.push(snd);
            }
        }
    }
}

impl Solver for BFSSolver {
    fn solve(mut self) -> Result<Vec<BoardMove>, ()> {
        if !is_solvable(&self.board) {
            return Err(());
        }

        self.perform_iteration()?;

        Ok(self.current_path)
    }
}
