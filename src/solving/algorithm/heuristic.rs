use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;

use crate::board::{Board, BoardMove, OwnedBoard};
use crate::solving::algorithm::heuristic::heuristics::Heuristic;
use crate::solving::algorithm::{util, Solver, SolvingError};
use crate::solving::is_solvable;
use crate::solving::movegen::MoveGenerator;

pub mod astar;
pub mod bestfs;
pub mod heuristics;

trait HeuristicSearchNode: Ord + Eq {
    fn create(board: OwnedBoard, heuristic: Rc<dyn Heuristic>) -> Self;
    fn with_path(board: OwnedBoard, path: Vec<BoardMove>, heuristic: Rc<dyn Heuristic>) -> Self;

    fn cost(&self) -> u64;
    fn desctructure(self) -> (OwnedBoard, Vec<BoardMove>);
}

struct HeuristicSolver<Node>
where
    Node: HeuristicSearchNode,
{
    heuristic: Rc<dyn Heuristic>,
    queue: BinaryHeap<Reverse<Node>>,
    move_generator: MoveGenerator,
}

impl<Node> HeuristicSolver<Node>
where
    Node: HeuristicSearchNode,
{
    #[must_use]
    pub fn new(board: OwnedBoard, heuristic: Box<dyn Heuristic>) -> Self {
        let mut queue = BinaryHeap::new();
        let heuristic: Rc<dyn Heuristic> = Rc::from(heuristic);
        if is_solvable(&board) {
            queue.push(Reverse(Node::create(board, Rc::clone(&heuristic))));
        }

        Self {
            heuristic,
            queue,
            move_generator: MoveGenerator::default(),
        }
    }

    fn visit_node(&mut self, node: Node) -> Option<Vec<BoardMove>> {
        let (board, path) = node.desctructure();

        if board.is_solved() {
            return Some(path);
        }

        for next_move in self
            .move_generator
            .generate_moves(&board, path.last().copied())
        {
            let mut new_board = board.clone();
            let mut new_path = path.clone();
            util::apply_move_sequence(&mut new_board, &mut new_path, next_move);
            self.queue.push(Reverse(Node::with_path(
                new_board,
                new_path,
                Rc::clone(&self.heuristic),
            )));
        }

        None
    }
}

impl<Node> Solver for HeuristicSolver<Node>
where
    Node: HeuristicSearchNode,
{
    fn solve(mut self: Box<Self>) -> Result<Vec<BoardMove>, SolvingError> {
        let mut max_cost = 0;
        while let Some(Reverse(node)) = self.queue.pop() {
            let cost = node.cost();
            if cost > max_cost {
                max_cost = cost;
                log::trace!("Evaluating position with cost {}", cost);
            }
            if let Some(result) = self.visit_node(node) {
                return Ok(result);
            }
        }
        Err(SolvingError::UnsolvableBoard)
    }
}
