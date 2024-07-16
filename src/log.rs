use clap_verbosity_flag::Verbosity;
use env_logger::Builder;
use log::{Level, LevelFilter};

pub fn init_logger(verbosity: Verbosity) {
    let mut log_builder = Builder::from_default_env();
    log_builder.filter(None, verbosity.log_level_filter());
    if verbosity.log_level() < Some(Level::Debug) {
        log_builder.filter(Some("actix_server"), LevelFilter::Warn);
    }
    log_builder.init();
}
