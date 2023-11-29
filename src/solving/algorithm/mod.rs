use crate::board::BoardMove;

pub mod bfs;
pub mod dfs;

pub trait Solver {
    fn solve(self) -> Result<Vec<BoardMove>, ()>;
}
