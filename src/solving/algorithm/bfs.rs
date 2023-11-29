use std::collections::{HashSet, VecDeque};

use crate::board::{Board, BoardMove, OwnedBoard};
use crate::solving::algorithm::Solver;
use crate::solving::is_solvable;
use crate::solving::movegen::{MoveGenerator, MoveSequence};

pub struct BFSSolver {
    visited: HashSet<OwnedBoard>,
    move_generator: MoveGenerator,
    queue: VecDeque<(OwnedBoard, Vec<BoardMove>)>,
}

impl BFSSolver {
    pub fn new(board: OwnedBoard, move_generator: MoveGenerator) -> Result<Self, ()> {
        if !is_solvable(&board) {
            return Err(());
        }

        let current_path = Vec::new();
        let queue = VecDeque::from(vec![(board.clone(), current_path.clone())]);

        Ok(Self {
            visited: HashSet::new(),
            move_generator,
            queue,
        })
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

    fn bfs_iteration(&mut self) -> Result<Option<Vec<BoardMove>>, ()> {
        if let Some((current_board, current_path)) = self.queue.pop_front() {
            if current_board.is_solved() {
                self.queue.clear();
                self.queue.push_back((current_board, current_path));
                return Ok(Some(self.queue.pop_front().unwrap().1));
            }

            if self.visited.contains(&current_board) {
                return Ok(None);
            }

            self.visited.insert(current_board.clone());

            for next_move in self.move_generator.generate_moves(&current_board, None) {
                let mut new_board = current_board.clone();
                let mut new_path = current_path.clone();
                self.exec_move_sequence(&mut new_board, &mut new_path, &next_move);
                self.queue.push_back((new_board, new_path));
            }

            Ok(None)
        } else {
            Ok(None)
        }
    }
}

impl Solver for BFSSolver {
    fn solve(mut self) -> Result<Vec<BoardMove>, ()> {

        while let Some(result) = self.bfs_iteration()? {
            return Ok(result);
        }

        Err(())
    }
}
