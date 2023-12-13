use std::cmp::Ordering;
use std::rc::Rc;

use crate::board::{Board, BoardMove, OwnedBoard};
use crate::solving::algorithm::heuristic::{HeuristicSearchNode, HeuristicSolver};
use crate::solving::algorithm::{util, Solver, SolvingError};
use crate::solving::is_solvable;
pub use crate::solving::movegen::MoveGenerator;

use super::heuristics::Heuristic;

struct SearchNode {
    board: OwnedBoard,
    path: Vec<BoardMove>,
    heuristic: Rc<dyn Heuristic>,
}

impl SearchNode {
    fn h_cost(&self) -> u64 {
        self.heuristic.evaluate(&self.board)
    }

    fn f_cost(&self) -> u64 {
        self.h_cost() + self.path.len() as u64
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
        self.f_cost().cmp(&other.f_cost())
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
        self.f_cost()
    }

    fn destructure(self) -> (OwnedBoard, Vec<BoardMove>) {
        let Self { board, path, .. } = self;
        (board, path)
    }
}

// OPTIMALITY
//
// This A* solver requires the heuristic to only be *admissible*,
// as it does the search on a tree, not a graph.
// As a consequence, it cannot implement search tree pruning in a simple way
pub struct AStarSolver {
    solver: HeuristicSolver<SearchNode>,
}

impl AStarSolver {
    #[must_use]
    pub fn new(board: OwnedBoard, heuristic: Box<dyn Heuristic>) -> Self {
        Self {
            solver: HeuristicSolver::new(board, heuristic),
        }
    }
}

impl Solver for AStarSolver {
    fn solve(self: Box<Self>) -> Result<Vec<BoardMove>, SolvingError> {
        Box::new(self.solver).solve()
    }
}

pub struct IterativeAStarSolver {
    heuristic: Box<dyn Heuristic>,
    path: Vec<BoardMove>,
    board: OwnedBoard,
    move_generator: MoveGenerator,
}

enum IDAStarResult {
    Ok,
    NotFound,
    Exceeded(u64),
}

impl IterativeAStarSolver {
    #[must_use]
    pub fn new(board: OwnedBoard, heuristic: Box<dyn Heuristic>) -> Self {
        Self {
            board,
            heuristic,
            path: vec![],
            move_generator: MoveGenerator::default(),
        }
    }

    fn search(&mut self, max_f_cost: u64) -> IDAStarResult {
        let f_cost = self.path.len() as u64 + self.heuristic.evaluate(&self.board);
        if f_cost > max_f_cost {
            return IDAStarResult::Exceeded(f_cost);
        }
        if self.board.is_solved() {
            return IDAStarResult::Ok;
        }
        let mut minimum = None;
        for next_move in self
            .move_generator
            .generate_moves(&self.board, self.path.last().copied())
        {
            util::apply_move_sequence(&mut self.board, &mut self.path, next_move);
            let result = self.search(max_f_cost);
            match (minimum, result) {
                (_, ok @ IDAStarResult::Ok) => return ok,
                (None, IDAStarResult::Exceeded(x)) => {
                    minimum = Some(x);
                }
                (Some(y), IDAStarResult::Exceeded(x)) if x < y => {
                    minimum = Some(x);
                }
                (_, _) => {}
            }
            util::undo_move_sequence(&mut self.board, &mut self.path, next_move);
        }
        minimum.map_or(IDAStarResult::NotFound, IDAStarResult::Exceeded)
    }
}

impl Solver for IterativeAStarSolver {
    fn solve(mut self: Box<Self>) -> Result<Vec<BoardMove>, SolvingError> {
        if !is_solvable(&self.board) {
            return Err(SolvingError::UnsolvableBoard);
        }
        let mut bound = self.heuristic.evaluate(&self.board);
        loop {
            match self.search(bound) {
                IDAStarResult::Ok => break Ok(self.path),
                IDAStarResult::NotFound => unreachable!("Should always return some heuristic"),
                IDAStarResult::Exceeded(x) => {
                    log::trace!("Increasing f-cost bound to {}", x);
                    bound = x;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

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

    #[test]
    fn board_with_shorter_path_gets_searched_first() {
        let board: OwnedBoard = r#"4 4
1 2 3 4
5 6 7 8
9 10 11 12
13 14 0 15"#
            .parse()
            .unwrap();

        let heuristic: Rc<dyn Heuristic> = Rc::new(heuristics::ManhattanDistance);
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(SearchNode {
            board: board.clone(),
            path: vec![],
            heuristic: Rc::clone(&heuristic),
        }));
        heap.push(Reverse(SearchNode {
            board: board.clone(),
            path: vec![BoardMove::Up],
            heuristic: Rc::clone(&heuristic),
        }));

        assert_eq!(
            0,
            heap.pop().expect("Heap should not be empty").0.path.len()
        );
        assert_eq!(
            1,
            heap.pop().expect("Heap should not be empty").0.path.len()
        );
    }
}
