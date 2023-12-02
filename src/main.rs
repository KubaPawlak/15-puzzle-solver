use clap::Parser;

use solver::board::BoardMove;
use solver::solving::algorithm::heuristics::{self, Heuristic};
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
    use heuristics::{InversionDistance, LinearConflict, ManhattanDistance};
    match heuristic_id {
        "MD" | "manhattan_distance" => Ok(Box::<ManhattanDistance>::default()),
        "LC" | "linear_conflict" => Ok(Box::<LinearConflict>::default()),
        "ID" | "inversion_distance" => Ok(Box::<InversionDistance>::default()),
        _ => Err("Unknown heuristic id. \
        Possible values are: MD, manhattan_distance, LC, linear_conflict, ID, inversion_distance."
            .to_string()),
    }
}

#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
#[clap(disable_help_flag = true)]
#[command(about, arg_required_else_help = true)]
struct CliArgs {
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

    #[arg(short, long, value_name = "HEURISTIC_ID", value_parser = crate::validate_heuristic, help = "Simplified Memory-bounded A*")]
    sma: Option<String>,
}

fn main() {
    let cli = CliArgs::parse();
    dbg!(cli);
}
