use solver::solving::algorithm::heuristic;
use solver::solving::algorithm::heuristic::astar::AStarSolver;

use crate::shared::{assert_produces_shortest_solution, assert_produces_valid_solution};

mod shared;

#[test]
fn produces_correct_solution() {
    assert_produces_valid_solution(|board| {
        AStarSolver::new(board, Box::new(heuristic::heuristics::ManhattanDistance))
    });
}

#[test]
fn produces_shortest_solution() {
    assert_produces_shortest_solution(|board| {
        AStarSolver::new(board, Box::new(heuristic::heuristics::ManhattanDistance))
    });
}
