use crate::board::Board;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Parity {
    Even,
    Odd,
}

pub fn calculate_parity<T>(permutation: &[T]) -> Parity
where
    T: Copy,
    usize: From<T>,
{
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

    let odd_cycle_count = cycle_lengths
        .into_iter()
        // we skip even cycles as they do not change the parity
        .filter(|len| len % 2 == 0) // cycle is odd if its length is even
        .count();

    if odd_cycle_count % 2 == 0 {
        // if there is an even number of odd cycles, they cancel out and the resulting parity is even
        Parity::Even
    } else {
        Parity::Odd
    }
}

pub fn solved_board_parity(board: &impl Board) -> Parity {
    let (rows, cols) = board.dimensions();
    let total_cells = rows * cols;

    // solved board is one big cycle, so parity is opposite its size
    if total_cells % 2 == 0 {
        Parity::Odd
    } else {
        Parity::Even
    }
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

    #[test]
    fn solved_board_has_inverse_parity_to_its_size() {
        let board_size = 16;
        let board_cells: Vec<u8> = (1..board_size).chain(once(0)).collect();

        let inverse_parity = if board_size % 2 == 0 {
            Parity::Odd
        } else {
            Parity::Even
        };

        assert_eq!(inverse_parity, calculate_parity(&board_cells));
    }
}
