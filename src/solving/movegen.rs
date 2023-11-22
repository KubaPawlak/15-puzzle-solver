use crate::board::BoardMove;

pub fn generate_pairs_of_moves(
    previous_pair: Option<(BoardMove, BoardMove)>,
) -> Vec<(BoardMove, BoardMove)> {
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
                if let Some((_prev_first, prev_second)) = previous_pair {
                    //Avoid rewinding previous move
                    if !is_opposite_move(first_move, prev_second) {
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
    matches!(
        (move1, move2),
        (BoardMove::Up, BoardMove::Down)
            | (BoardMove::Down, BoardMove::Up)
            | (BoardMove::Left, BoardMove::Right)
            | (BoardMove::Right, BoardMove::Left)
    )
}
