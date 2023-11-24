use crate::board::{Board, BoardMove};
use crate::solving::movegen::MoveSequence::Single;
use crate::solving::parity;
use crate::solving::parity::Parity;

pub enum MoveSequence {
    Single(BoardMove),
    Double(BoardMove, BoardMove),
}

/// Generates the next moves to perform on the board.
/// This function only generates moves that can be executed on a board, and have the proper parity
pub fn next_moves(board: &impl Board, previous_move: Option<BoardMove>) -> Vec<MoveSequence> {
    use MoveSequence::Double;

    let moves = [
        BoardMove::Up,
        BoardMove::Down,
        BoardMove::Left,
        BoardMove::Right,
    ];
    let mut next_moves = Vec::new();

    let generate_single_move = parity::required_moves_parity(board) == Parity::Odd;

    for first_move in moves {
        let first_position = position_after_move(board.empty_cell_pos(), first_move);
        if !is_inside_board(first_position, board) {
            // cannot execute move
            continue;
        }
        if let Some(previous_move) = previous_move {
            if first_move == previous_move.opposite() {
                // move would undo the previous move
                continue;
            }
        }

        if generate_single_move {
            next_moves.push(Single(first_move))
        } else {
            for second_move in moves {
                let second_position = position_after_move(first_position, second_move);
                if !is_inside_board(second_position, board) {
                    // second move is impossible to execute
                    continue;
                }
                // Avoid obviously unsound moves
                if second_move != first_move.opposite() {
                    next_moves.push(Double(first_move, second_move));
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        if generate_single_move {
            assert!(next_moves
                .iter()
                .all(|m| matches!(m, MoveSequence::Single(_))));
        } else {
            assert!(next_moves
                .iter()
                .all(|m| matches!(m, MoveSequence::Double(_, _))));
        }
    }

    next_moves
}

/// Helper function to check where the empty square would move, to ensure that the move is able to be performed
fn position_after_move(initial_position: (u8, u8), board_move: BoardMove) -> (u8, u8) {
    let (row, col) = initial_position;
    match board_move {
        BoardMove::Up => (row - 1, col),
        BoardMove::Down => (row + 1, col),
        BoardMove::Left => (row, col - 1),
        BoardMove::Right => (row, col + 1),
    }
}

fn is_inside_board(position: (u8, u8), board: &impl Board) -> bool {
    let (row, col) = position;
    let (rows, columns) = board.dimensions();

    row > 0 && col > 0 && row < rows && col < columns
}
