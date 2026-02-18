use nono::Result;
use std::fs;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};

use crate::Args;

pub fn init_tracing(args: &Args) {
    let crate_level = if args.debug { "debug" } else { "info" };

    // Open (or create) the log file
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // use .truncate(true) if you want a fresh file each run
        .open("log/app.log")
        .expect("failed to open log file");

    let file_filter = EnvFilter::builder().parse_lossy(crate_level);
    tracing::debug!("[Filter] Log file = {file_filter}");

    let file_layer = tracing_subscriber::fmt::layer()
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

pub fn init_logging(args: &Args) -> Result<()> {
    init_tracing(args);

    // Setup eyre
    // color_eyre::install()
    //     .map_err(|err| Error::Custom(format!("Failed to setup eyre: {err:#?}")))?;

    Ok(())
}

