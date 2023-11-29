use std::collections::{HashSet, VecDeque};

use crate::board::{BoardMove, OwnedBoard};
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
}
