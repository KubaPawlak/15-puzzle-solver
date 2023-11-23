use std::ops::Add;

use crate::board::Board;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Parity {
    Even,
    Odd,
}

impl Parity {
    fn opposite(&self) -> Parity {
        match self {
            Parity::Even => Parity::Odd,
            Parity::Odd => Parity::Even,
        }
    }
}

impl From<usize> for Parity {
    fn from(value: usize) -> Self {
        if value % 2 == 0 {
            Parity::Even
        } else {
            Parity::Odd
        }
    }
}

impl Add for Parity {
    type Output = Parity;

    fn add(self, rhs: Self) -> Self::Output {
        use Parity::*;

        match (self, rhs) {
            (Even, Even) | (Odd, Odd) => Even,
            (_, _) => Odd,
        }
    }
}

pub fn calculate_parity<T: Into<usize> + Copy>(permutation: &[T]) -> Parity {
    let mut visited = bit_set::BitSet::with_capacity(permutation.len());
    let mut cycle_lengths = vec![];

    for &element in permutation {
        let mut element: usize = element.into();
        let mut cycle_length: usize = 0;

        while !visited.contains(element) {
            // visit whole cycle
            visited.insert(element);
            element = permutation[element].into();
            cycle_length += 1;
        }

        if cycle_length > 1 {
            // not interested in 1-cycles
            cycle_lengths.push(cycle_length);
        }
    }

    cycle_lengths
        .into_iter()
        .map(|len| Parity::from(len).opposite()) // parity of a cycle is opposite of the parity of its length
        .fold(Parity::Even, Parity::add)
}

pub fn solved_board_parity(board: &impl Board) -> Parity {
    let (rows, cols) = board.dimensions();
    let total_cells = rows as usize * cols as usize;

    // solved board is one big cycle, so parity is opposite its size
    Parity::from(total_cells).opposite()
}

#[cfg(test)]
mod test {
    use std::iter::once;

    use super::*;

    #[test]
    fn odd_permutation_has_odd_parity() {
        let odd_permutation = [2u8, 3, 4, 1, 0];
        assert_eq!(Parity::Odd, calculate_parity(&odd_permutation));
    }

    #[test]
    fn even_permutation_has_even_parity() {
        let even_permutation = [0u8, 1, 4, 2, 3];
        assert_eq!(Parity::Even, calculate_parity(&even_permutation));
    }

    mod solved_board_has_inverse_parity_to_its_size {
        use super::*;

        fn solved_board_has_inverse_parity_to_its_size(rows: usize, columns: usize) {
            let board_size = rows * columns;
            let board_cells: Vec<u8> = (1..board_size as u8).chain(once(0)).collect();

            let inverse_parity = Parity::from(board_size).opposite();

            assert_eq!(inverse_parity, calculate_parity(&board_cells));
        }

        macro_rules! test_cases {
            ($($name:ident ($rows:expr, $cols:expr)),*) => {
                $(
                    #[test]
                    fn $name (){
                        solved_board_has_inverse_parity_to_its_size($rows, $cols)
                    }
                )*
            };
        }

        test_cases! {
            board_2x2 (2,2),
            board_2x3 (2,3),
            board_2x4 (2,4),
            board_2x5 (2,5),
            board_3x2 (3,2),
            board_3x3 (3,3),
            board_3x4 (3,4),
            board_3x5 (3,5),
            board_4x2 (4,2),
            board_4x3 (4,3),
            board_4x4 (4,4),
            board_4x5 (4,5),
            board_5x2 (5,2),
            board_5x3 (5,3),
            board_5x4 (5,4),
            board_5x5 (5,5)
        }
    }
}
