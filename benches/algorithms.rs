use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use solver::solving::algorithm::{
    dfs::{DFSSolver, IncrementalDFSSolver},
    Solver,
};
use solver::solving::movegen::MoveGenerator;

mod shared;

pub fn solver_algorithms_benchmark(c: &mut Criterion) {
    let board = shared::create_sample_board();

    c.bench_function("DFS", |b| {
        b.iter_batched(
            || {
                Box::new(DFSSolver::new(
                    black_box(board.clone()),
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
                    black_box(board.clone()),
                    MoveGenerator::default(),
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
