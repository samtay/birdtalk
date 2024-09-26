use dioxus::prelude::*;

use crate::{
    stats::Stats,
    ui::{
        components::{Login, Modal},
        game::GameCtx,
        pages::PLAY_STATUS,
        AppCtx, Route,
    },
};

#[component]
pub fn GameOverModal() -> Element {
    let game_over_dismissed_cb = use_callback(|_| {
        spawn(async move {
            // Let modal animation slide out of frame before dropping this screen.
            // tracing::debug!("Game over! Sleeping for 0.5s...");
            // #[cfg(feature = "web")]
            // async_std::task::sleep(std::time::Duration::from_millis(500)).await;
            tracing::debug!("Game over! Resetting game status...");
            *PLAY_STATUS.write() = None;
            navigator().push(Route::Birds {});
        });
    });

    let _auth = use_context::<AppCtx>().auth_state;

    rsx! {
        Modal {
            on_dismiss: game_over_dismissed_cb,
            div {
                class: "p-2 sm:p-4 mx-auto my-2 flex flex-col items-center gap-4 sm:gap-8 text-center",
                h1 {
                    class: "text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-green to-green-extra-dark uppercase",
                    "Nice work!"
                }
                table { class: "table-auto text-lg",
                    tbody {
                        Stat { name: "XP", f: Stats::xp }
                        Stat { name: "Birds Learned", f: Stats::total_birds_learned }
                        Stat { name: "Daily Pack Streak", f: Stats::active_daily_pack_streak, fprev: Some(Stats::latest_daily_pack_streak)}
                    }
                }
                // TODO: uncomment when auth is fully implemented
                // if auth.is_logged_in() {
                if true {
                    button {
                        class: "px-4 py-2 border-2 border-green-extra-dark focus:outline-none focus-visible:ring focus-visible:ring-green-dark font-semibold text-base bg-green-dark text-white rounded-xl shadow sm:hover:scale-110 sm:hover:shadow-xl sm:hover:bg-gradient-to-r from-green to-green-dark transition-transform text-xl",
                        // TODO: this handler doesn't have access to internal modal visibility
                        // signal, that's why slide down doesn't work.
                        // ... among other reasons.
                        onclick: move |_| {
                            game_over_dismissed_cb.call(())
                        },
                        onmounted: move |mnt| async move {
                            #[cfg(feature = "web")]
                            async_std::task::sleep(std::time::Duration::from_millis(500)).await;
                            mnt.set_focus(true).await.ok();
                        },
                        "Continue"
                    }
                } else {
                    div {
                        h3 {
                            class: "mb-2 text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-green to-green-extra-dark",
                            "Sign up to save your progress!"
                        }
                        Login {}
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
struct StatProps {
    name: &'static str,
    f: fn(&Stats) -> u32,
    /// Optionally specify a different function to get the previous stats value.
    #[props(default = None)]
    fprev: Option<fn(&Stats) -> u32>,
}

fn Stat(StatProps { name, f, fprev }: StatProps) -> Element {
    let game_ctx = use_context::<GameCtx>();
    let stats = game_ctx.stats.read();
    let og_stats = game_ctx.stats_original.read();

    let value = f(&stats);
    let og_value = if let Some(fp) = fprev {
        fp(&og_stats)
    } else {
        f(&og_stats)
    };
    let change = value.checked_sub(og_value).filter(|v| *v > 0);
    rsx! {
        tr {
            td { class: "text-right px-1", "{name}:" }
            td { class: "text-left px-1", "{value}"  }
            if let Some(change) = change {
                td { class: "text-left px-2 text-green-dark font-bold", "+{change}"}
            }
        }
    }
}
