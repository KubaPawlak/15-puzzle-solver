use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use solver::solving::algorithm::heuristics::ManhattanDistance;
use solver::solving::algorithm::{solvers::*, Solver};
use solver::solving::movegen::MoveGenerator;

mod shared;

pub fn solver_algorithms_benchmark(c: &mut Criterion) {
    let mut boards = shared::create_sample_boards();

    c.bench_function("DFS", |b| {
        b.iter_batched(
            || {
                Box::new(DFSSolver::new(
                    black_box(boards.next().unwrap()),
                    MoveGenerator::default(),
                ))
            },
            |solver| {
                let _ = black_box(solver.solve());
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("IDFS", |b| {
        b.iter_batched(
            || {
                Box::new(IncrementalDFSSolver::new(
                    black_box(boards.next().unwrap()),
                    MoveGenerator::default(),
                ))
            },
            |solver| {
                let _ = black_box(solver.solve());
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("BFS", |b| {
        b.iter_batched(
            || {
                Box::new(BFSSolver::new(
                    black_box(boards.next().unwrap()),
                    MoveGenerator::default(),
                ))
            },
            |solver| {
                let _ = black_box(solver.solve());
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("A*", |b| {
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
}

criterion_group!(algorithm_benchmarks, solver_algorithms_benchmark);
criterion_main!(algorithm_benchmarks);
