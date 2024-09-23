#![allow(dead_code)] // TODO: remove
#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod bird;
mod conf;
mod pack;
mod stats;
mod supabase;
mod sync;
mod ui;
mod utils;

fn main() {
    // Init storage
    dioxus_sdk::set_dir!();

    // Init logger
    let log_level = option_env!("RUST_LOG")
        .and_then(|level| level.parse().ok())
        .unwrap_or(Level::INFO);
    dioxus_logger::init(log_level).expect("failed to init logger");

    // Launch the app
    launch(ui::App)
}
