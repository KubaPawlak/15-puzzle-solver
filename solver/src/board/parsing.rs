use super::Board;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

impl FromStr for Board {
    type Err = BoardCreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let (rows, columns) = {
            let first_line = lines
                .next()
                .ok_or(BoardCreationError::InvalidHeader)?
                .split_whitespace()
                .collect::<Vec<_>>();

            if first_line.len() != 2 {
                return Err(BoardCreationError::InvalidHeader);
            }

            let parsed: Vec<u8> = first_line
                .into_iter()
                .map(str::parse)
                .collect::<Result<_, _>>()?;

            (parsed[0], parsed[1])
        };

        let mut cells = vec![0u8; rows as usize * columns as usize];

        let mut row_count: usize = 0;
        for (board_row, input_line) in cells
            .chunks_mut(columns as usize)
            .zip(lines.take(rows as usize))
        {
            let values: Vec<u8> = input_line
                .split_whitespace()
                .take(columns as usize)
                .map(str::parse)
                .collect::<Result<_, _>>()?;

            debug_assert!(board_row.len() == columns as usize);
            if values.len() != board_row.len() {
                return Err(BoardCreationError::MissingCells);
            }
            board_row.clone_from_slice(&values);

            row_count += 1;
        }

        if row_count != rows as usize {
            return Err(BoardCreationError::MissingCells);
        }

        for i in 0..=(columns * rows - 1) {
            match cells.iter().copied().filter(|&x| x == i).count().cmp(&1) {
                Ordering::Less => return Err(BoardCreationError::MissingCells),
                Ordering::Greater => return Err(BoardCreationError::DuplicateCells),
                Ordering::Equal => {}
            }
        }

        Ok(Self {
            rows,
            columns,
            cells,
        })
    }
}

pub(crate) enum BoardCreationError {
    ParsingError(ParseIntError),
    InvalidHeader,
    MissingCells,
    DuplicateCells,
}

impl From<ParseIntError> for BoardCreationError {
    fn from(value: ParseIntError) -> Self {
        BoardCreationError::ParsingError(value)
    }
}

impl Display for BoardCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardCreationError::ParsingError(err) => {
                write!(f, "Error while parsing board: {err}")
            }
            BoardCreationError::MissingCells => write!(
                f,
                "The board does not contain all of the required cell values"
            ),
            BoardCreationError::DuplicateCells => {
                write!(f, "The board contains multiple cells with the same number")
            }
            BoardCreationError::InvalidHeader => write!(f, "The size header is invalid or missing"),
        }
    }
}

impl Debug for BoardCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <BoardCreationError as Display>::fmt(self, f)
    }
}

impl Error for BoardCreationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BoardCreationError::ParsingError(err) => Some(err),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLVED_INPUT: &str = r"4 4
1  2  3  4
5  6  7  8
9 10 11 12
13 14 15 0
";

    #[test]
    fn board_correctly_parsed() {
        let board: Board = SOLVED_INPUT.parse().unwrap();

        assert_eq!(board.dimensions(), (4, 4));

        assert_eq!(board.at(0, 0), 1);
        assert_eq!(board.at(0, 1), 2);
        assert_eq!(board.at(0, 2), 3);
        assert_eq!(board.at(0, 3), 4);
        assert_eq!(board.at(1, 0), 5);
        assert_eq!(board.at(1, 1), 6);
        assert_eq!(board.at(1, 2), 7);
        assert_eq!(board.at(1, 3), 8);
        assert_eq!(board.at(2, 0), 9);
        assert_eq!(board.at(2, 1), 10);
        assert_eq!(board.at(2, 2), 11);
        assert_eq!(board.at(2, 3), 12);
        assert_eq!(board.at(3, 0), 13);
        assert_eq!(board.at(3, 1), 14);
        assert_eq!(board.at(3, 2), 15);
        assert_eq!(board.at(3, 3), 0);
    }
}
