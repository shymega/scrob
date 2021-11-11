use slog::{Drain, Level, LevelFilter, Logger};
use slog_term::{FullFormat, PlainDecorator};
use std::io::stdout;

use slog_async::Async;

pub fn initialise_logger(lvl: Level) -> Logger {
    let decorator = PlainDecorator::new(stdout());
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = LevelFilter::new(drain, lvl).fuse();
    let drain = Async::new(drain).build().fuse();

    Logger::root(drain, o!())
}
