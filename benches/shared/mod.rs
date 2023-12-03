use itertools::Itertools;
use solver::board::{BoardMove, OwnedBoard};
use solver::solving::movegen::SearchOrder;

pub fn create_sample_boards() -> impl Iterator<Item = OwnedBoard> {
    let board_strings = vec![
        r"3 3
2 4 0
1 6 3
7 5 8
",
    ];

    let boards: Vec<_> = board_strings
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    InfiniteIterator {
        inner: boards,
        current_index: 0,
    }
}

struct InfiniteIterator<T> {
    inner: Vec<T>,
    current_index: usize,
}

impl<T> Iterator for InfiniteIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner[self.current_index].clone();
        self.current_index += 1;
        if self.current_index >= self.inner.len() {
            self.current_index = 0;
        }
        Some(value)
    }
}

pub fn generate_all_search_orders() -> Vec<SearchOrder> {
    let search_orders: Vec<SearchOrder> = [
        BoardMove::Up,
        BoardMove::Down,
        BoardMove::Left,
        BoardMove::Right,
    ]
    .into_iter()
    .permutations(4)
    .map(|p| SearchOrder::Provided([p[0], p[1], p[2], p[3]]))
    // .chain(std::iter::once(SearchOrder::Random))
    .collect();
    search_orders
}
