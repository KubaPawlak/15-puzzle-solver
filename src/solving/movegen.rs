use crate::board::BoardMove;

pub fn generate_pairs_of_moves(previous_move: Option<BoardMove>) -> Vec<(BoardMove, BoardMove)> {
    let moves = [
        BoardMove::Up,
        BoardMove::Down,
        BoardMove::Left,
        BoardMove::Right,
    ];
    let mut pairs = Vec::new();

    for &first_move in &moves {
        for &second_move in &moves {
            // Avoid obviously unsound moves
            if !is_opposite_move(first_move, second_move) {
                if let Some(prev_move) = previous_move {
                    //Avoid rewinding previous move
                    if !is_opposite_move(first_move, prev_move) {
                        pairs.push((first_move, second_move));
                    }
                } else {
                    pairs.push((first_move, second_move))
                }
            }
        }
    }
    pairs
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
