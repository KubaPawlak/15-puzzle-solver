#![allow(dead_code)]

use crate::board::Board;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub(crate) struct VisitedPositions<T: Board + Eq + Hash> {
    visited_states: Arc<RwLock<HashSet<T>>>,
}

impl<T: Board + Eq + Hash> VisitedPositions<T> {
    pub(crate) fn new() -> Self {
        VisitedPositions {
            // Arc allows multiple threads
            visited_states: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    // Check if a board state has been visited
    pub(crate) fn is_visited(&self, board: &T) -> bool {
        let lock = self.visited_states.read().expect("RwLock read lock");
        lock.contains(board)
    }

    // Mark a board state as visited
    pub(crate) fn mark_visited(&self, board: T) {
        let mut lock = self.visited_states.write().expect("RwLock write lock");
        lock.insert(board);
    }

    pub(crate) fn clear(&self) {
        let mut lock = self.visited_states.write().expect("RwLock write lock");
        lock.clear();
    }
}
