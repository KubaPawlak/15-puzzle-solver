use std::error::Error;
use std::fmt::{Display, Formatter};

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

#[derive(Debug)]
pub enum SolvingError {
    UnsolvableBoard,
    AlgorithmError(Box<dyn Error>),
}

impl Display for SolvingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SolvingError::UnsolvableBoard => write!(f, "Board is unsolvable"),
            SolvingError::AlgorithmError(inner) => {
                write!(f, "Solving error: {}", inner)
            }
        }
    }
}

impl Error for SolvingError {}

pub trait Solver {
    fn solve(self: Box<Self>) -> Result<Vec<BoardMove>, SolvingError>;
}
