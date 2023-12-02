use crate::board::{Board, BoardMove};
use crate::solving::parity;
use crate::solving::parity::Parity;

#[derive(Clone, Debug)]
pub enum MoveSequence {
    Single(BoardMove),
    Double(BoardMove, BoardMove),
}

#[derive(Clone, Debug)]
pub enum SearchOrder {
    Provided([BoardMove; 4]),
    Random,
}

pub struct MoveGenerator {
    search_order: SearchOrder,
}

impl Default for MoveGenerator {
    fn default() -> Self {
        use crate::board::BoardMove::*;
        Self::new(SearchOrder::Provided([Up, Down, Left, Right]))
    }
}

impl MoveGenerator {
    pub fn new(search_order: SearchOrder) -> Self {
        MoveGenerator { search_order }
    }

    pub fn generate_moves(
        &self,
        board: &impl Board,
        previous_move: Option<BoardMove>,
    ) -> Vec<MoveSequence> {
        let mut next_moves = Vec::new();

        let generate_single_move = parity::required_moves_parity(board) == Parity::Odd;

        let search_order = match self.search_order {
            SearchOrder::Provided(order) => order,
            SearchOrder::Random => todo!("Handle random move generation"),
        };

        for first_move in search_order {
            let empty_pos = board.empty_cell_pos();
            let first_position =
                position_after_move((empty_pos.0 as i16, empty_pos.1 as i16), first_move);
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
                next_moves.push(MoveSequence::Single(first_move))
            } else {
                for second_move in search_order {
                    let second_position = position_after_move(first_position, second_move);
                    if !is_inside_board(second_position, board) {
                        // second move is impossible to execute
                        continue;
                    }
                    // Avoid obviously unsound moves
                    if second_move != first_move.opposite() {
                        next_moves.push(MoveSequence::Double(first_move, second_move));
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
}

/// Helper function to check where the empty square would move, to ensure that the move is able to be performed
fn position_after_move(initial_position: (i16, i16), board_move: BoardMove) -> (i16, i16) {
    let (row, col) = initial_position;
    match board_move {
        BoardMove::Up => (row - 1, col),
        BoardMove::Down => (row + 1, col),
        BoardMove::Left => (row, col - 1),
        BoardMove::Right => (row, col + 1),
    }
}

fn is_inside_board(position: (i16, i16), board: &impl Board) -> bool {
    let (row, col) = position;
    let (rows, columns) = board.dimensions();
    let (rows, columns) = (rows as i16, columns as i16);

    row >= 0 && col >= 0 && row < rows && col < columns
}

#[cfg(test)]
mod test {
    use crate::board::{Board, BoardMove, OwnedBoard};
    use crate::solving::parity::{required_moves_parity, Parity};

    use super::{MoveGenerator, MoveSequence};

    const SOLVED_INPUT: &str = r"4 4
1  2  3  4
5  6  7  8
9 10 11 12
13 14 15 0
";

    #[test]
    fn generates_single_moves_with_odd_board() {
        let mut board = SOLVED_INPUT.parse::<OwnedBoard>().unwrap();

        // make 3 moves, so board should require an odd number to solve
        board.exec_move(BoardMove::Up);
        board.exec_move(BoardMove::Up);
        board.exec_move(BoardMove::Left);

        let move_generator = MoveGenerator::default();

        let next_moves = move_generator.generate_moves(&board, None);

        assert!(next_moves
            .iter()
            .all(|m| matches!(m, MoveSequence::Single(_))))
    }

    #[test]
    fn generates_double_moves_with_even_board() {
        let mut board = SOLVED_INPUT.parse::<OwnedBoard>().unwrap();

        // make 3 moves, so board should require an odd number to solve
        board.exec_move(BoardMove::Up);
        board.exec_move(BoardMove::Left);

        let move_generator = MoveGenerator::default();

        let next_moves = move_generator.generate_moves(&board, None);

        assert!(next_moves
            .iter()
            .all(|m| matches!(m, MoveSequence::Double(_, _))))
    }

    #[test]
    fn does_generate_all_moves_that_can_be_executed() {
        use BoardMove::*;
        let mut board = SOLVED_INPUT.parse::<OwnedBoard>().unwrap();
        assert_eq!((4, 4), board.dimensions());

        let path: Vec<_> = vec![
            Up, Up, Up, Left, Left, Left, Down, Down, Down, Right, Right, Right,
        ];

        let all_moves = [Up, Down, Left, Right];

        let move_generator = MoveGenerator::default();

        for path_move in path {
            board.exec_move(path_move);

            let next_moves: Vec<_> = move_generator
                .generate_moves(&board, None)
                .into_iter()
                .map(|m| match m {
                    MoveSequence::Single(x) => x,
                    MoveSequence::Double(x, _) => x,
                })
                .collect();

            for m in all_moves {
                if board.can_move(m) {
                    assert!(next_moves.contains(&m))
                }
            }
        }
    }

    #[test]
    fn second_moves_can_always_be_executed() {
        use BoardMove::*;
        let mut board = SOLVED_INPUT.parse::<OwnedBoard>().unwrap();
        assert_eq!((4, 4), board.dimensions());

        let path: Vec<_> = vec![
            Up, Up, Up, Left, Left, Left, Down, Down, Down, Right, Right, Right,
        ];

        let move_generator = MoveGenerator::default();

        for path_move in path {
            board.exec_move(path_move);

            if required_moves_parity(&board) != Parity::Even {
                continue;
            }

            for next_move in move_generator.generate_moves(&board, None) {
                match next_move {
                    MoveSequence::Single(_) => unreachable!(),
                    MoveSequence::Double(fst, snd) => {
                        board.exec_move(fst);
                        assert!(board.can_move(snd));
                        board.exec_move(fst.opposite());
                    }
                }
            }
        }
    }
}
