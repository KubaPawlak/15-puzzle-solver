use solver::board::OwnedBoard;
use solver::solving::algorithm::bfs::BFSSolver;
use solver::solving::algorithm::Solver;
use solver::solving::movegen::MoveGenerator;

mod shared;

#[test]
fn produces_correct_solution() {
    shared::assert_produces_valid_solution(|b| BFSSolver::new(b, MoveGenerator::default()));
}

#[test]
fn produces_shortest_solution() {
    shared::assert_produces_shortest_solution(|b| BFSSolver::new(b, MoveGenerator::default()))
}
