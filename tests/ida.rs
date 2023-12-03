use solver::solving::algorithm::astar::IterativeAStarSolver;
use solver::solving::algorithm::heuristics;

use crate::shared::{assert_produces_shortest_solution, assert_produces_valid_solution};

mod shared;

#[test]
fn produces_correct_solution() {
    assert_produces_valid_solution(|board| {
        IterativeAStarSolver::new(board, Box::new(heuristics::ManhattanDistance))
    });
}

#[test]
fn produces_shortest_solution() {
    assert_produces_shortest_solution(|board| {
        IterativeAStarSolver::new(board, Box::new(heuristics::ManhattanDistance))
    });
}
