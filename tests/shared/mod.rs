use solver::board::{Board, BoardMove, OwnedBoard};

pub fn is_valid_solution(mut board: OwnedBoard, solution: Vec<BoardMove>) -> bool {
    for m in solution {
        board.exec_move(m);
    }

    board.is_solved()
}
