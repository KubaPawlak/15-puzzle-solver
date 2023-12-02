use solver::solving::algorithm::astar::AStarSolver;
use solver::solving::algorithm::heuristics;

use crate::shared::{assert_produces_shortest_solution, assert_produces_valid_solution};

mod shared;

#[test]
fn produces_correct_solution() {
    assert_produces_valid_solution(|board| AStarSolver::new(board, heuristics::ManhattanDistance));
}

#[test]
fn produces_shortest_solution() {
    assert_produces_shortest_solution(|board| {
        AStarSolver::new(board, heuristics::ManhattanDistance)
    });
}
