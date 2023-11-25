use crate::board::BoardMove;

pub mod dfs;

pub trait Solver {
    fn solve(self) -> Result<Vec<BoardMove>, ()>;
}
