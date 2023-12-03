use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use solver::solving::algorithm::heuristics::{
    InversionDistance, LinearConflict, ManhattanDistance,
};
use solver::solving::algorithm::{solvers::*, Solver};
use solver::solving::movegen::MoveGenerator;

mod shared;

pub fn solver_algorithms_benchmark(c: &mut Criterion) {
    let mut boards = shared::create_sample_boards();

    let search_orders = shared::generate_all_search_orders();

    let mut uniform_group = c.benchmark_group("Uniform search");
    for order in search_orders {
        uniform_group.bench_function(BenchmarkId::new("DFS", order.clone()), |b| {
            b.iter_batched(
                || {
                    Box::new(DFSSolver::new(
                        black_box(boards.next().unwrap()),
                        MoveGenerator::new(order.clone()),
                    ))
                },
                |solver| {
                    let _ = black_box(solver.solve());
                },
                BatchSize::SmallInput,
            )
        });
        uniform_group.bench_function(BenchmarkId::new("IDFS", order.clone()), |b| {
            b.iter_batched(
                || {
                    Box::new(IncrementalDFSSolver::new(
                        black_box(boards.next().unwrap()),
                        MoveGenerator::new(order.clone()),
                    ))
                },
                |solver| {
                    let _ = black_box(solver.solve());
                },
                BatchSize::SmallInput,
            )
        });

        uniform_group.bench_function(BenchmarkId::new("BFS", order.clone()), |b| {
            b.iter_batched(
                || {
                    Box::new(BFSSolver::new(
                        black_box(boards.next().unwrap()),
                        MoveGenerator::new(order.clone()),
                    ))
                },
                |solver| {
                    let _ = black_box(solver.solve());
                },
                BatchSize::SmallInput,
            )
        });
    }
    uniform_group.finish();

    let mut heuristic_group = c.benchmark_group("Heuristic");
    heuristic_group.bench_function(BenchmarkId::new("A star", "Manhattan distance"), |b| {
        b.iter_batched(
            || {
                Box::new(AStarSolver::new(
                    black_box(boards.next().unwrap()),
                    Box::<ManhattanDistance>::default(),
                ))
            },
            |solver| {
                let _ = black_box(solver.solve());
            },
            BatchSize::SmallInput,
        )
    });
    heuristic_group.bench_function(BenchmarkId::new("A star", "Linear conflict"), |b| {
        b.iter_batched(
            || {
                Box::new(AStarSolver::new(
                    black_box(boards.next().unwrap()),
                    Box::<LinearConflict>::default(),
                ))
            },
            |solver| {
                let _ = black_box(solver.solve());
            },
            BatchSize::SmallInput,
        )
    });
    heuristic_group.bench_function(BenchmarkId::new("A star", "Inversion distance"), |b| {
        b.iter_batched(
            || {
                Box::new(AStarSolver::new(
                    black_box(boards.next().unwrap()),
                    Box::<InversionDistance>::default(),
                ))
            },
            |solver| {
                let _ = black_box(solver.solve());
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    name = extensive_benchmarks;
    config = Criterion::default().sample_size(50);
    targets = solver_algorithms_benchmark
);
criterion_main!(extensive_benchmarks);
