use solver::board::OwnedBoard;
use solver::solving::algorithm::{dfs::DFSSolver, Solver};

mod shared;

#[test]
fn produces_correct_solution() {
    let board_str = r#"3 3
1 2 3
0 4 6
7 5 8
"#;

    let board: OwnedBoard = board_str.parse().unwrap();

    let solver = DFSSolver::new(board);

    let solution = solver.solve().expect("Board is unsolvable");

    let solution_str: Vec<_> = solution.iter().map(|x| x.to_string()).collect();

    let solution_str = solution_str.join("");

    eprintln!("Solution length {}", solution.len());
    eprintln!("{solution_str}");

    let is_valid = shared::is_valid_solution(board_str.parse().unwrap(), solution);
    assert!(is_valid, "Solution produced is not valid")
}
