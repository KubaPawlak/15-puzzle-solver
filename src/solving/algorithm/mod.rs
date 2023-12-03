use crate::board::BoardMove;

pub mod astar;
pub mod bfs;
pub mod dfs;
pub mod heuristics;

pub mod solvers {
    pub use super::astar::AStarSolver;
    pub use super::astar::IterativeAStarSolver;
    pub use super::bfs::BFSSolver;
    pub use super::dfs::DFSSolver;
    pub use super::dfs::IncrementalDFSSolver;
}

pub trait Solver {
    fn solve(self: Box<Self>) -> Result<Vec<BoardMove>, ()>;
}
