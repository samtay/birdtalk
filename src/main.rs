#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod bird;
mod game;
mod ui;

// These are automagically included in the <head>.
// Note that URLs are relative to your Cargo.toml file.
// const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    // launch(App)
    // use below until --port 3000 supported on fullstack
    #[allow(clippy::let_unit_value)]
    let cfg = server_only!(
        dioxus::fullstack::Config::new().addr(std::net::SocketAddr::from(([0, 0, 0, 0], 3000)))
    );
    LaunchBuilder::fullstack().with_cfg(cfg).launch(ui::App)
}
