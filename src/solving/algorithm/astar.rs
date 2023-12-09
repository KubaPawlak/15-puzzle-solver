use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;

use crate::board::{Board, BoardMove, OwnedBoard};
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

// OPTIMALITY
//
// This A* solver requires the heuristic to only be *admissible*,
// as it does the search on a tree, not a graph.
// As a consequence, it cannot implement search tree pruning in a simple way
pub struct AStarSolver {
    heuristic: Rc<dyn Heuristic>,
    queue: BinaryHeap<Reverse<SearchNode>>,
    move_generator: MoveGenerator,
}

impl AStarSolver {
    #[must_use]
    pub fn new(board: OwnedBoard, heuristic: Box<dyn Heuristic>) -> Self {
        let mut queue = BinaryHeap::new();
        let heuristic: Rc<dyn Heuristic> = Rc::from(heuristic);
        if is_solvable(&board) {
            queue.push(Reverse(SearchNode {
                board,
                path: vec![],
                heuristic: Rc::clone(&heuristic),
            }));
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
            util::apply_move_sequence(&mut new_board, &mut new_path, next_move);
            self.queue.push(Reverse(SearchNode {
                board: new_board,
                path: new_path,
                heuristic: Rc::clone(&self.heuristic),
            }));
        }

        None
    }
}

impl Solver for AStarSolver {
    fn solve(mut self: Box<Self>) -> Result<Vec<BoardMove>, SolvingError> {
        let mut max_f_cost = 0;
        while let Some(Reverse(node)) = self.queue.pop() {
            let f_cost = node.f_cost();
            if f_cost > max_f_cost {
                max_f_cost = f_cost;
                log::trace!("Evaluating position with f-cost {}", f_cost);
            }
            if let Some(result) = self.visit_node(node) {
                return Ok(result);
            }
        }
        Err(SolvingError::UnsolvableBoard)
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

struct SMAStarNode {
    board: OwnedBoard,
    path: Vec<BoardMove>,
    f_cost: u64,
    best_forgotten_child: Option<u64>,
}

impl SMAStarNode {
    #[must_use]
    pub fn new(board: OwnedBoard, path: Vec<BoardMove>, heuristic: &dyn Heuristic) -> Self {
        let f_cost = heuristic.evaluate(&board) + path.len() as u64;
        Self {
            board,
            path,
            f_cost,
            best_forgotten_child: None,
        }
    }
}

impl PartialEq for SMAStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board && self.path == other.path
    }
}

impl Eq for SMAStarNode {}

impl PartialOrd for SMAStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SMAStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.f_cost.cmp(&other.f_cost) {
            // if the f-costs are the same, the deeper node is better
            Ordering::Equal => self.path.len().cmp(&other.path.len()).reverse(),
            other => other,
        }
    }
}

// NOTE: May not work
pub struct MemoryBoundedAStarSolver {
    queue: VecDeque<SMAStarNode>,
    heuristic: Rc<dyn Heuristic>,
    move_generator: MoveGenerator,
    memory_limit: Option<usize>,
}

impl MemoryBoundedAStarSolver {
    #[must_use]
    pub fn new(board: OwnedBoard, heuristic: Box<dyn Heuristic>) -> Self {
        let mut queue = VecDeque::new();
        let heuristic: Rc<dyn Heuristic> = Rc::from(heuristic);
        if is_solvable(&board) {
            queue.push_back(SMAStarNode::new(board, vec![], &*heuristic));
        }

        Self {
            heuristic,
            queue,
            move_generator: MoveGenerator::default(),
            memory_limit: None,
        }
    }

    #[must_use]
    pub fn with_memory_limit(
        board: OwnedBoard,
        heuristic: Box<dyn Heuristic>,
        memory_limit: usize,
    ) -> Self {
        let mut queue = VecDeque::new();
        let heuristic: Rc<dyn Heuristic> = Rc::from(heuristic);
        if is_solvable(&board) {
            queue.push_back(SMAStarNode::new(board, vec![], &*heuristic));
        }

        Self {
            heuristic,
            queue,
            move_generator: MoveGenerator::default(),
            memory_limit: Some(memory_limit),
        }
    }

    fn children(&self, node: &SMAStarNode) -> impl IntoIterator<Item = SMAStarNode> {
        let board = &node.board;
        let path = &node.path;
        let mut children = vec![];

        for next_move in self
            .move_generator
            .generate_moves(board, path.last().copied())
        {
            let mut new_board = board.clone();
            let mut new_path = path.clone();
            util::apply_move_sequence(&mut new_board, &mut new_path, next_move);
            children.push(SMAStarNode::new(new_board, new_path, &*self.heuristic));
        }

        children
    }

    fn enqueue(&mut self, node: SMAStarNode) {
        // instead of linear search, we can do binary search, since we know that the queue is ordered
        let insert_index = match self.queue.binary_search(&node) {
            Ok(i) => i,
            Err(i) => i,
        };

        self.queue.insert(insert_index, node);
    }

    fn reduce_memory(&mut self) {
        let deleted = self
            .queue
            .pop_back()
            .expect("If memory is full then queue should have nodes");

        if let Some(parent) = self.find_parent(&deleted) {
            parent.best_forgotten_child = Some(
                parent
                    .best_forgotten_child
                    .map_or(deleted.f_cost, |x| u64::min(x, deleted.f_cost)),
            )
        }
    }

    fn find_parent(&mut self, node: &SMAStarNode) -> Option<&mut SMAStarNode> {
        self.queue
            .iter()
            .enumerate()
            .filter(|(_, n)| node.path.starts_with(&n.path))
            .max_by(|(_, fst), (_, snd)| fst.path.len().cmp(&snd.path.len()))
            .map(|(i, _)| i)
            .and_then(|i| self.queue.get_mut(i))
    }

    fn visit_node(&mut self, mut node: SMAStarNode) -> Option<Vec<BoardMove>> {
        if node.board.is_solved() {
            return Some(node.path);
        }

        let next_child: Option<SMAStarNode> = self
            .children(&node)
            .into_iter()
            .find(|c| !self.queue.contains(c));

        if let Some(next_child) = next_child {
            if self.is_memory_full() {
                self.reduce_memory()
            }
            self.enqueue(next_child);

            if self.is_memory_full() {
                self.reduce_memory()
            }
            self.enqueue(node);
        } else {
            node.f_cost = self
                .children(&node)
                .into_iter()
                .map(|c| {
                    self.queue
                        .iter()
                        .find(|&m| *m == c)
                        .expect("Children should be in memory")
                })
                .map(|c| c.f_cost)
                .min()
                .unwrap_or(node.f_cost);

            if self.is_memory_full() {
                self.reduce_memory()
            }
            self.enqueue(node)
        }

        None
    }

    fn is_memory_full(&self) -> bool {
        self.memory_limit
            .is_some_and(|limit| self.queue.len() >= limit)
    }
}

impl Solver for MemoryBoundedAStarSolver {
    fn solve(mut self: Box<Self>) -> Result<Vec<BoardMove>, SolvingError> {
        while let Some(node) = self.queue.pop_front() {
            if let Some(result) = self.visit_node(node) {
                return Ok(result);
            }
        }
        Err(SolvingError::UnsolvableBoard)
    }
}

#[cfg(test)]
mod tests {
    use crate::solving::algorithm::heuristics;

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
        heap.push(SearchNode {
            board: board.clone(),
            path: vec![],
            heuristic: Rc::clone(&heuristic),
        });
        heap.push(SearchNode {
            board: board.clone(),
            path: vec![BoardMove::Up],
            heuristic: Rc::clone(&heuristic),
        });

        assert_eq!(0, heap.pop().expect("Heap should not be empty").path.len());
        assert_eq!(1, heap.pop().expect("Heap should not be empty").path.len());
    }
}
