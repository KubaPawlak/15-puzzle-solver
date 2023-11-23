use crate::board::{Board, BoardMove};

enum MoveSequence {
    Single(BoardMove),
    Double(BoardMove, BoardMove),
}

pub fn next_moves(_board: &impl Board, previous_move: Option<BoardMove>) -> Vec<MoveSequence> {
    use MoveSequence::Double;

    let moves = [
        BoardMove::Up,
        BoardMove::Down,
        BoardMove::Left,
        BoardMove::Right,
    ];
    let mut next_moves = Vec::new();

    let generate_single_move = false; // todo: Check parity of the required number of moves for board and generate appropriate number of moves

    for first_move in moves {
        if generate_single_move {
            unimplemented!()
        } else {
            for second_move in moves {
                // Avoid obviously unsound moves
                if !is_opposite_move(first_move, second_move) {
                    if let Some(prev_move) = previous_move {
                        //Avoid rewinding previous move
                        if !is_opposite_move(first_move, prev_move) {
                            next_moves.push(Double(first_move, second_move));
                        }
                    } else {
                        next_moves.push(Double(first_move, second_move))
                    }
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

/// Helper function to check if two moves are opposites (e.g., Up and Down)
fn is_opposite_move(move1: BoardMove, move2: BoardMove) -> bool {
    use BoardMove::*;
    matches!(
        (move1, move2),
        (Up, Down) | (Down, Up) | (Left, Right) | (Right, Left)
    )
}
/// Helper function to generate single moves. Needed to be used first if number of steps in solution is odd.
pub fn generate_single_moves() -> Vec<BoardMove> {
    vec![
        BoardMove::Up,
        BoardMove::Down,
        BoardMove::Left,
        BoardMove::Right,
    ]
}
