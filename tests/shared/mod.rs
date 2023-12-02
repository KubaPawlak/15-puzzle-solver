use solver::board::{Board, BoardMove, OwnedBoard};
use solver::solving::algorithm::Solver;

fn is_valid_solution(mut board: OwnedBoard, solution: Vec<BoardMove>) -> bool {
    for m in solution {
        board.exec_move(m);
    }

    board.is_solved()
}

fn solution_to_string(solution: &[BoardMove]) -> String {
    let solution_str: Vec<_> = solution.iter().map(|x| x.to_string()).collect();
    solution_str.join("")
}

const TEST_DATA: &[(&str, usize)] = &[(
    r#"3 3
1 2 3
0 4 6
7 5 8
"#,
    3,
)];

fn generate_test_data() -> Vec<(OwnedBoard, usize)> {
    TEST_DATA
        .iter()
        .map(|(brd, len)| (brd.parse().unwrap(), *len))
        .collect()
}

pub fn assert_produces_valid_solution<S: Solver>(mut solver_builder: impl FnMut(OwnedBoard) -> S) {
    let test_data = generate_test_data();

    for (board, _shortest_solution) in test_data {
        let solver = Box::new(solver_builder(board.clone()));
        let solution = solver.solve().expect("board should be solvable");

        eprintln!("Solution length {}", solution.len());
        eprintln!("{}", solution_to_string(&solution));

        assert!(is_valid_solution(board, solution));
    }
}

pub fn assert_produces_shortest_solution<S: Solver>(
    mut solver_builder: impl FnMut(OwnedBoard) -> S,
) {
    let test_data = generate_test_data();

    for (board, shortest_solution) in test_data {
        let solver = Box::new(solver_builder(board.clone()));
        let solution = solver.solve().expect("board should be solvable");

        eprintln!("Solution length {}", solution.len());
        eprintln!("{}", solution_to_string(&solution));

        assert_eq!(solution.len(), shortest_solution);
    }
}
