use solver::board::OwnedBoard;
use solver::solving::algorithm::{dfs::IncrementalDFSSolver, Solver};

mod shared;

#[test]
fn produces_correct_solution() {
    let board_str = r#"3 3
1 2 3
0 4 6
7 5 8
"#;

    let board: OwnedBoard = board_str.parse().unwrap();

    let solver = IncrementalDFSSolver::new(board);

    let solution = solver.solve().expect("Board is unsolvable");

    eprintln!("Solution length {}", solution.len());
    eprintln!("{}", shared::solution_to_string(&solution));

    let is_valid = shared::is_valid_solution(board_str.parse().unwrap(), solution);
    assert!(is_valid, "Solution produced is not valid")
}

#[test]
fn produces_shortest_solution() {
    let board_str = r#"3 3
1 2 3
0 4 6
7 5 8
"#;

    let board: OwnedBoard = board_str.parse().unwrap();

    let solver = IncrementalDFSSolver::new(board);

    let solution = solver.solve().expect("Board is unsolvable");

    assert_eq!(3, solution.len(), "Solution is not the shortest one")
}
