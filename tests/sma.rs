use solver::solving::algorithm::astar::MemoryBoundedAStarSolver;
use solver::solving::algorithm::heuristics::ManhattanDistance;

use crate::shared::{assert_produces_shortest_solution, assert_produces_valid_solution};

mod shared;

#[test]
fn produces_correct_solution() {
    assert_produces_valid_solution(|board| {
        MemoryBoundedAStarSolver::with_memory_limit(board, Box::<ManhattanDistance>::default(), 20)
    });
}

#[test]
fn produces_shortest_solution() {
    assert_produces_shortest_solution(|board| {
        MemoryBoundedAStarSolver::with_memory_limit(board, Box::<ManhattanDistance>::default(), 20)
    });
}
