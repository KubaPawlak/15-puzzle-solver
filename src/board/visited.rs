use crate::board::{Board, BoardMove};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

struct VisitedPositions {
    visited_states: HashSet<dyn Board>,
}

impl VisitedPositions {
    fn new() -> Self {
        VisitedPositions {
            visited_states: HashSet::new(),
        }
    }

    // Check if a board state has been visited
    fn is_visited(&self, board: &dyn Board) -> bool {
        self.visited_states.contains(board)
    }

    // Mark a board state as visited
    fn mark_visited(&mut self, board: Box<dyn Board>) {
        self.visited_states.insert(board);
    }

}