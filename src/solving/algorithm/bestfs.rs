use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

use crate::board::{Board, BoardMove, OwnedBoard};
use crate::solving::algorithm::{heuristics, Solver, SolvingError};
use crate::solving::is_solvable;
use crate::solving::movegen::{MoveGenerator, MoveSequence};

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
        self.h_cost().cmp(&other.h_cost()).reverse() // reverse the ordering so that board with lower heuristic shows as greater
    }
}

pub struct BestFSSolver {
    heuristic: Rc<dyn Heuristic>,
    queue: BinaryHeap<SearchNode>,
    move_generator: MoveGenerator,
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

fn undo_move_sequence(
    board: &mut impl Board,
    path: &mut Vec<BoardMove>,
    move_sequence: MoveSequence,
) {
    match move_sequence {
        MoveSequence::Single(m) => {
            board.exec_move(m.opposite());
            path.pop();
        }
        MoveSequence::Double(fst, snd) => {
            board.exec_move(snd.opposite());
            board.exec_move(fst.opposite());
            path.pop();
            path.pop();
        }
    }
}

impl BestFSSolver {
    #[must_use]
    pub fn new(board: OwnedBoard, heuristic: Box<dyn Heuristic>) -> Self {
        let mut queue = BinaryHeap::new();
        let heuristic: Rc<dyn Heuristic> = Rc::from(heuristic);
        if is_solvable(&board) {
            queue.push(SearchNode {
                board,
                path: vec![],
                heuristic: Rc::clone(&heuristic),
            });
        }

        Self {
            heuristic,
            queue,
            move_generator: MoveGenerator::default(),
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
            apply_move_sequence(&mut new_board, &mut new_path, next_move);
            self.queue.push(SearchNode {
                board: new_board,
                path: new_path,
                heuristic: Rc::clone(&self.heuristic),
            });
        }

        None
    }
}

impl Solver for BestFSSolver {
    fn solve(mut self: Box<Self>) -> Result<Vec<BoardMove>, SolvingError> {
        let mut max_h_cost = 0;
        while let Some(node) = self.queue.pop() {
            let h_cost = node.h_cost();
            if h_cost > max_h_cost {
                max_h_cost = h_cost;
                log::trace!("Evaluating position with h-cost {}", h_cost);
            }
            if let Some(result) = self.visit_node(node) {
                return Ok(result);
            }
        }
        Err(SolvingError::UnsolvableBoard)
    }
}

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
    heap.push(SearchNode {
        board: simple_board.clone(),
        path: vec![],
        heuristic: Rc::clone(&heuristic),
    });
    heap.push(SearchNode {
        board: worse_board.clone(),
        path: vec![],
        heuristic: Rc::clone(&heuristic),
    });

    assert_eq!(
        simple_board,
        heap.pop().expect("Heap should not be empty").board
    );
    assert_eq!(
        worse_board,
        heap.pop().expect("Heap should not be empty").board
    );
}
