use dioxus::prelude::*;

use crate::{
    stats::Stats,
    ui::{components::Modal, game::GameCtx, GameStatus, GAME_STATUS},
};

#[component]
pub fn GameOverModal() -> Element {
    let game_over_dismissed_cb = use_callback(|| {
        spawn(async move {
            // Let modal animation slide out of frame before dropping this screen.
            // tracing::debug!("Game over! Sleeping for 0.5s...");
            // #[cfg(feature = "web")]
            // async_std::task::sleep(std::time::Duration::from_millis(500)).await;
            tracing::debug!("Game over! Resetting game status...");
            *GAME_STATUS.write() = GameStatus::None;
        });
    });

    rsx! {
        Modal {
            on_dismiss: game_over_dismissed_cb,
            div {
                class: "p-2 sm:p-4 mx-auto my-2 flex flex-col items-center gap-4 sm:gap-8 text-center",
                h1 {
                    class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-green-600 to-green-800",
                    "Nice work!"
                }
                table { class: "table-auto text-lg",
                    tbody {
                        Stat { name: "XP", f: Stats::xp }
                        Stat { name: "Birds Learned", f: Stats::birds_learned }
                    }
                }
                button {
                    class: "px-4 py-2 focus:outline-none focus-visible:ring focus-visible:ring-green-400 font-semibold text-base bg-green-800 text-amber-50 rounded-full shadow",
                    // TODO: this handler doesn't have access to internal modal visibility
                    // signal, that's why slide down doesn't work.
                    // ... among other reasons.
                    onclick: move |_| {
                        game_over_dismissed_cb.call()
                    },
                    "Continue"
                }
            }
        }
    }
}

#[component]
fn Stat(name: &'static str, f: fn(&Stats) -> u32) -> Element {
    let game_ctx = use_context::<GameCtx>();
    let stats = game_ctx.stats.read();
    let og_stats = game_ctx.stats_original.read();

    let value = f(&stats);
    let og_value = f(&og_stats);
    let change = value.checked_sub(og_value).filter(|v| *v > 0);
    rsx! {
        tr {
            td { class: "text-right px-1", "{name}:" }
            td { class: "text-left px-1", "{value}", }
            if let Some(change) = change {
                td { class: "text-left px-1 text-sm text-green-400 font-semibold", "+{change}"}
            }
        }
    }
}
