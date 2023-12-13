use solver::solving::algorithm::heuristic;
use solver::solving::algorithm::heuristic::astar::IterativeAStarSolver;

use crate::shared::{assert_produces_shortest_solution, assert_produces_valid_solution};

mod shared;

#[test]
fn produces_correct_solution() {
    assert_produces_valid_solution(|board| {
        IterativeAStarSolver::new(board, Box::new(heuristic::heuristics::ManhattanDistance))
    });
}

#[test]
fn produces_shortest_solution() {
    assert_produces_shortest_solution(|board| {
        IterativeAStarSolver::new(board, Box::new(heuristic::heuristics::ManhattanDistance))
    });
}
