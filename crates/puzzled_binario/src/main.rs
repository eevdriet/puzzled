use std::fs;

use puzzled_binario::{BinarioSolver, Puzzle, binario};
use tracing_subscriber::{
    EnvFilter, Layer, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

fn main() {
    init_tracing();

    let puzzle = binario!(
        [ - - - - - ]
        [ 0 - - 0 - ]
        [ - 1 - 0 - ]
        [ - 1 1 - - ]
        [ - - - 1 - ]
    );
    let mut solver = BinarioSolver::default();

    let _ = puzzle.solve_with(&mut solver).expect("To solve");
}

pub fn init_tracing() {
    // Open (or create) the log file
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // use .truncate(true) if you want a fresh file each run
        .open("log/app.log")
        .expect("failed to open log file");

    let file_filter = EnvFilter::builder().parse_lossy("debug");
    tracing::debug!("[Filter] Log file = {file_filter}");

    let file_layer = fmt::layer()
        .with_thread_ids(false)
        .with_target(false)
        .with_ansi(false)
        .with_writer(file)
        .compact()
        .with_filter(file_filter);

    // let stdout_filter = EnvFilter::builder().parse_lossy("info");

    // let stdout_layer = tracing_subscriber::fmt::layer()
    //     .with_thread_ids(false)
    //     .with_target(false)
    //     .with_ansi(true)
    //     .with_writer(std::io::stdout)
    //     .compact()
    //     .with_filter(stdout_filter);

    Registry::default()
        .with(file_layer)
        // .with(stdout_layer)
        .init();
}
