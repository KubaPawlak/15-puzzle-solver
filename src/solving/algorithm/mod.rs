use crate::board::BoardMove;

pub mod bfs;
pub mod dfs;
mod heuristics;

pub trait Solver {
    fn solve(self) -> Result<Vec<BoardMove>, ()>;
}
