use std::cmp::Ordering;
use std::rc::Rc;

use crate::board::{BoardMove, OwnedBoard};
use crate::solving::algorithm::heuristic::heuristics::Heuristic;
use crate::solving::algorithm::heuristic::{HeuristicSearchNode, HeuristicSolver};
use crate::solving::algorithm::{Solver, SolvingError};

pub struct SearchNode {
    board: OwnedBoard,
    path: Vec<BoardMove>,
    heuristic: Rc<dyn Heuristic>,
}

impl SearchNode {
    fn h_cost(&self) -> u64 {
        self.heuristic.evaluate(&self.board)
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board && self.path == other.path
    }
}

impl Eq for SearchNode {}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.h_cost().cmp(&other.h_cost())
    }
}

impl HeuristicSearchNode for SearchNode {
    fn create(board: OwnedBoard, heuristic: Rc<dyn Heuristic>) -> Self {
        Self {
            board,
            path: vec![],
            heuristic,
        }
    }

    fn with_path(board: OwnedBoard, path: Vec<BoardMove>, heuristic: Rc<dyn Heuristic>) -> Self {
        Self {
            board,
            path,
            heuristic,
        }
    }

    fn cost(&self) -> u64 {
        self.h_cost()
    }

    fn destructure(self) -> (OwnedBoard, Vec<BoardMove>) {
        let Self { board, path, .. } = self;
        (board, path)
    }
}

pub struct BestFSSolver {
    solver: HeuristicSolver<SearchNode>,
}

impl BestFSSolver {
    pub fn new(board: OwnedBoard, heuristic: Box<dyn Heuristic>) -> Self {
        Self {
            solver: HeuristicSolver::new(board, heuristic),
        }
    }
}

impl Solver for BestFSSolver {
    fn solve(self: Box<Self>) -> Result<Vec<BoardMove>, SolvingError> {
        Box::new(self.solver).solve()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    use crate::board::Board;
    use crate::solving::algorithm::heuristic::heuristics;

    use super::*;

    #[test]
    fn board_with_lower_heuristic_gets_searched_first() {
        let simple_board: OwnedBoard = r#"4 4
1 2 3 4
5 6 7 8
9 10 11 12
13 14 0 15"#
            .parse()
            .unwrap();
        let mut worse_board = simple_board.clone();
        worse_board.exec_move(BoardMove::Up);

        let heuristic: Rc<dyn Heuristic> = Rc::new(heuristics::ManhattanDistance);
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(SearchNode {
            board: simple_board.clone(),
            path: vec![],
            heuristic: Rc::clone(&heuristic),
        }));
        heap.push(Reverse(SearchNode {
            board: worse_board.clone(),
            path: vec![],
            heuristic: Rc::clone(&heuristic),
        }));

        assert_eq!(
            simple_board,
            heap.pop().expect("Heap should not be empty").0.board
        );
        assert_eq!(
            worse_board,
            heap.pop().expect("Heap should not be empty").0.board
        );
    }
}
