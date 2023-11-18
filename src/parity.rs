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

#[cfg(test)]
mod test {
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
}
