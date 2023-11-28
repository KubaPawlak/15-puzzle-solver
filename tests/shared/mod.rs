use solver::board::{Board, BoardMove, OwnedBoard};

pub fn is_valid_solution(mut board: OwnedBoard, solution: Vec<BoardMove>) -> bool {
    for m in solution {
        board.exec_move(m);
    }

    board.is_solved()
}

pub fn solution_to_string(solution: &[BoardMove]) -> String {
    let solution_str: Vec<_> = solution.iter().map(|x| x.to_string()).collect();
    solution_str.join("")
}
