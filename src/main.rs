use clap::Parser;
use log::LevelFilter;

use solver::board::{BoardMove, OwnedBoard};
use solver::solving::algorithm::heuristic::heuristics::{
    Heuristic, InversionDistance, LinearConflict, ManhattanDistance,
};
use solver::solving::algorithm::{Solver, SolvingError};
use solver::solving::movegen::SearchOrder;

fn parse_search_order(s: &str) -> Result<SearchOrder, String> {
    const ORDER_LEN: usize = 4;
    let input = s.to_uppercase();
    if input == "R" {
        Ok(SearchOrder::Random)
    } else if input.len() != ORDER_LEN {
        Err(format!("Order must be {ORDER_LEN} characters"))
    } else {
        let order: Vec<BoardMove> = input
            .chars()
            .map(|c| match c {
                'U' => Ok(BoardMove::Up),
                'D' => Ok(BoardMove::Down),
                'L' => Ok(BoardMove::Left),
                'R' => Ok(BoardMove::Right),
                _ => Err(format!("Invalid character {c}")),
            })
            .collect::<Result<_, _>>()?;

        for i in 1..ORDER_LEN {
            let current = &order[i - 1];
            if order[i..].contains(current) {
                return Err(format!("Duplicate move {current}"));
            }
        }

        Ok(SearchOrder::Provided([
            order[0], order[1], order[2], order[3],
        ]))
    }
}

fn validate_heuristic(heuristic_id: &str) -> Result<String, String> {
    parse_heuristic(heuristic_id)?;
    Ok(heuristic_id.to_string())
}

fn parse_heuristic(heuristic_id: &str) -> Result<Box<dyn Heuristic>, String> {
    match heuristic_id {
        "MD" | "manhattan_distance" => Ok(Box::<ManhattanDistance>::default()),
        "LC" | "linear_conflict" => Ok(Box::<LinearConflict>::default()),
        "ID" | "inversion_distance" => Ok(Box::<InversionDistance>::default()),
        _ => Err("Unknown heuristic id. \
        Possible values are: MD, manhattan_distance, LC, linear_conflict, ID, inversion_distance."
            .to_string()),
    }
}

#[derive(Parser, Debug, Clone)]
struct CliArgs {
    #[clap(flatten)]
    algorithm_info: AlgorithmArgs,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Parser, Clone, Debug)]
#[group(required = true, multiple = false)]
#[clap(disable_help_flag = true)]
#[command(about, arg_required_else_help = true)]
struct AlgorithmArgs {
    #[arg(short, long, value_name = "ORDER", value_parser = crate::parse_search_order, help="Breadth-first search")]
    bfs: Option<SearchOrder>,

    #[arg(short, long, value_name = "ORDER", value_parser = crate::parse_search_order, help = "Depth-first search")]
    dfs: Option<SearchOrder>,

    #[arg(short, long, value_name = "ORDER", value_parser = crate::parse_search_order, help = "Iterative deepening DFS")]
    idfs: Option<SearchOrder>,

    #[arg(short = 'h', long = "bf", value_name = "HEURISTIC_ID", value_parser = crate::validate_heuristic, help = "Greedy Best-first search")]
    best_first: Option<String>,

    #[arg(short, long, value_name = "HEURISTIC_ID", value_parser = crate::validate_heuristic, help = "A* search algorithm")]
    astar: Option<String>,

    #[arg(long, value_name = "HEURISTIC_ID", value_parser = crate::validate_heuristic, help = "A* search algorithm")]
    ida: Option<String>,
}

fn create_solver(config: AlgorithmArgs, board: OwnedBoard) -> Box<dyn Solver> {
    use solver::solving::algorithm::solvers::*;
    use solver::solving::movegen::MoveGenerator;

    if let Some(order) = config.bfs {
        Box::new(BFSSolver::new(board, MoveGenerator::new(order)))
    } else if let Some(order) = config.dfs {
        Box::new(DFSSolver::new(board, MoveGenerator::new(order)))
    } else if let Some(order) = config.idfs {
        Box::new(IncrementalDFSSolver::new(board, MoveGenerator::new(order)))
    } else if let Some(heuristic_id) = &config.best_first {
        let _heuristic = parse_heuristic(heuristic_id)
            .expect("Parser should fail if heuristic id was incorrect");
        todo!("Best-first search is not implemented yet")
    } else if let Some(heuristic_id) = &config.astar {
        let heuristic = parse_heuristic(heuristic_id)
            .expect("Parser should fail if heuristic id was incorrect");
        Box::new(AStarSolver::new(board, heuristic))
    } else if let Some(heuristic_id) = &config.ida {
        let heuristic = parse_heuristic(heuristic_id)
            .expect("Parser should fail if heuristic id was incorrect");
        Box::new(IterativeAStarSolver::new(board, heuristic))
    } else {
        unreachable!("Parser should fail if none of the options are selected")
    }
}

fn main() {
    let cli = CliArgs::parse();

    simple_logger::SimpleLogger::new()
        .with_local_timestamps()
        .with_timestamp_format(time::macros::format_description!(
            "[hour]:[minute]:[second]"
        ))
        .init()
        .unwrap();

    log::set_max_level(match cli.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3.. => LevelFilter::Trace,
    });

    let board = match OwnedBoard::try_from_iter(
        std::io::stdin()
            .lines()
            .map(|l| l.expect("Stdin must be valid UTF-8")),
    ) {
        Ok(board) => board,
        Err(e) => {
            log::error!("Error while parsing board: {e}");
            std::process::exit(1);
        }
    };

    let solver = create_solver(cli.algorithm_info, board);
    log::info!("Starting solver");

    let start = std::time::Instant::now();
    let solve_result = solver.solve();
    let finish = start.elapsed();
    let solution = match solve_result {
        Ok(solution) => {
            log::info!(
                "Found solution in {:#}",
                duration_human::DurationHuman::from(finish)
            );
            solution
        }
        Err(SolvingError::UnsolvableBoard) => {
            log::warn!("Board is unsolvable");
            Vec::default()
        }
        Err(SolvingError::AlgorithmError(inner_error)) => {
            log::error!("Unable to solve board: {}", inner_error);
            std::process::exit(1);
        }
    };

    println!("{}", solution.len());
    let solution_str: Vec<_> = solution
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    println!("{}", solution_str.join(""));
}
