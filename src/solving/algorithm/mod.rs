use crate::board::BoardMove;

pub mod astar;
pub mod bfs;
pub mod dfs;
pub mod heuristics;

pub trait Solver {
    fn solve(self) -> Result<Vec<BoardMove>, ()>;
}
