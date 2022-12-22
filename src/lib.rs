// #![feature(allocator_api)]
// #![feature(collections)]
mod logging;
mod allocator;
mod cache;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate slog_json;
extern crate lazy_static;
extern crate core;
extern crate cty;
extern crate bumpalo;

use lazy_static::lazy_static;
use slog::Logger;

use crate::logging::logging::initialize_logging;

lazy_static! {
    static ref LOGGER: Logger = initialize_logging(String::from("dynamic_re_reference_cache_"));
}

fn main() {
    info!(&crate::LOGGER, "Configured logging");
}
