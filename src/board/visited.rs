use crate::board::{Board};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

struct VisitedPositions {
    visited_states: Arc<RwLock<HashSet<Box<dyn Board>>>>,
}

impl VisitedPositions {
    fn new() -> Self {
        VisitedPositions {
            /// Arc allows multiple threads
            visited_states: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    // Check if a board state has been visited
    fn is_visited(&self, board: &dyn Board) -> bool {
        let lock = self.visited_states.read().expect("RwLock read lock");
        lock.contains(board)
    }

    // Mark a board state as visited
    fn mark_visited(&self, board: Box<dyn Board>) {
        let mut lock = self.visited_states.write().expect("RwLock write lock");
        lock.insert(board);
    }
}