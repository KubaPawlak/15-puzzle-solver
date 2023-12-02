use solver::solving::algorithm::dfs::IncrementalDFSSolver;
use solver::solving::movegen::MoveGenerator;

use crate::shared::{assert_produces_shortest_solution, assert_produces_valid_solution};

mod shared;

#[test]
fn produces_correct_solution() {
    assert_produces_valid_solution(|board| {
        IncrementalDFSSolver::new(board, MoveGenerator::default())
    });
}

#[test]
fn produces_shortest_solution() {
    assert_produces_shortest_solution(|board| {
        IncrementalDFSSolver::new(board, MoveGenerator::default())
    });
}
