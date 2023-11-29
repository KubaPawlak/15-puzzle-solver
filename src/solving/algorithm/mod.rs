use crate::board::BoardMove;

pub mod dfs;
pub mod bfs;

pub trait Solver {
    fn solve(self) -> Result<Vec<BoardMove>, ()>;
}
