use std::collections::VecDeque;

use crate::board::{Board, BoardMove, OwnedBoard};
use crate::solving::algorithm::{util, Solver, SolvingError};
use crate::solving::is_solvable;
use crate::solving::movegen::MoveGenerator;
use crate::solving::visited::VisitedPositions;

pub struct BFSSolver {
    visited_positions: VisitedPositions<OwnedBoard>,
    move_generator: MoveGenerator,
    queue: VecDeque<(OwnedBoard, Vec<BoardMove>)>,
}

impl BFSSolver {
    #[must_use]
    pub fn new(board: OwnedBoard, move_generator: MoveGenerator) -> Self {
        let mut queue = VecDeque::new();
        if is_solvable(&board) {
            queue.push_back((board, Vec::new()));
        }
        Self {
            visited_positions: VisitedPositions::new(),
            move_generator,
            queue,
        }
    }

    fn bfs_iteration(
        &mut self,
        current_board: &OwnedBoard,
        current_path: &[BoardMove],
    ) -> Option<Vec<BoardMove>> {
        if current_board.is_solved() {
            return Some(current_path.to_vec());
        }

        if self.visited_positions.is_visited(current_board) {
            return None;
        }

        self.visited_positions.mark_visited(current_board.clone());

        for next_move in self.move_generator.generate_moves(current_board, None) {
            let mut new_board = current_board.clone();
            let mut new_path = current_path.to_vec();
            util::apply_move_sequence(&mut new_board, &mut new_path, next_move);
            self.queue.push_back((new_board, new_path));
        }

        None
    }
}

impl Solver for BFSSolver {
    fn solve(mut self: Box<Self>) -> Result<Vec<BoardMove>, SolvingError> {
        while let Some((board, path)) = self.queue.pop_front() {
            if let Some(result) = self.bfs_iteration(&board, &path) {
                return Ok(result);
            }
        }
        Err(SolvingError::UnsolvableBoard)
    }
}
