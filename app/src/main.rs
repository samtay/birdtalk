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
    // launch(App)
    // use below until --port 3000 supported on fullstack
    #[allow(clippy::let_unit_value)]
    // let cfg = server_only!(
    //     dioxus::fullstack::Config::new().addr(std::net::SocketAddr::from(([0, 0, 0, 0], 3000)))
    // );
    // LaunchBuilder::fullstack().with_cfg(cfg).launch(ui::App)
    launch(ui::App)
}
