use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

use crate::board::{Board, BoardMove, OwnedBoard};
use crate::solving::algorithm::Solver;
use crate::solving::is_solvable;
use crate::solving::movegen::{MoveGenerator, MoveSequence};

use super::heuristics::Heuristic;

struct SearchNode {
    board: OwnedBoard,
    path: Vec<BoardMove>,
    heuristic: Rc<dyn Heuristic>,
}

impl SearchNode {
    fn heuristic(&self) -> u64 {
        self.heuristic.evaluate(&self.board) + self.path.len() as u64
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
        let self_score = self.heuristic() + self.path.len() as u64;
        let other_score = other.heuristic() + other.path.len() as u64;

        self_score.cmp(&other_score).reverse() // reverse the ordering so that board with lower heuristic shows as greater
    }
}

pub struct AStarSolver {
    heuristic: Rc<dyn Heuristic>,
    queue: BinaryHeap<SearchNode>,
    move_generator: MoveGenerator,
}

impl AStarSolver {
    pub fn new(board: OwnedBoard, heuristic: impl Heuristic + 'static) -> Self {
        let mut queue = BinaryHeap::new();
        let heuristic: Rc<dyn Heuristic> = Rc::new(heuristic);
        if is_solvable(&board) {
            queue.push(SearchNode {
                board,
                path: vec![],
                heuristic: Rc::clone(&heuristic),
            })
        }

        Self {
            heuristic,
            queue,
            move_generator: MoveGenerator::default(),
        }
    }

    fn apply_move_sequence(
        board: &mut impl Board,
        path: &mut Vec<BoardMove>,
        move_sequence: MoveSequence,
    ) {
        match move_sequence {
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

    fn visit_node(&mut self, SearchNode { board, path, .. }: SearchNode) -> Option<Vec<BoardMove>> {
        if board.is_solved() {
            return Some(path);
        }

        for next_move in self
            .move_generator
            .generate_moves(&board, path.last().copied())
        {
            let mut new_board = board.clone();
            let mut new_path = path.clone();
            Self::apply_move_sequence(&mut new_board, &mut new_path, next_move);
            self.queue.push(SearchNode {
                board: new_board,
                path: new_path,
                heuristic: Rc::clone(&self.heuristic),
            });
        }

        None
    }
}

impl Solver for AStarSolver {
    fn solve(mut self) -> Result<Vec<BoardMove>, ()> {
        while let Some(node) = self.queue.pop() {
            if let Some(result) = self.visit_node(node) {
                return Ok(result);
            }
        }
        Err(())
    }
}
