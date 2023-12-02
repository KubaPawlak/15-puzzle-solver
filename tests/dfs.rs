use solver::solving::algorithm::dfs::DFSSolver;
use solver::solving::movegen::MoveGenerator;

use crate::shared::assert_produces_valid_solution;

mod shared;

#[test]
fn produces_correct_solution() {
    assert_produces_valid_solution(|board| DFSSolver::new(board, MoveGenerator::default()));
}
