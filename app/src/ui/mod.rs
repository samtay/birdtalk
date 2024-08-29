//! In [`crate::ui`] we have all of the actual markup that makes up the app. I.e. usage of rsx!
//! lives here.

mod components;
mod game;
mod pages;

use dioxus::prelude::*;

use crate::{
    bird::BirdPack,
    stats::Stats,
    supabase::AuthState,
    sync::Sync,
    ui::pages::{Birds, Index, Play},
};

pub static PLAY_STATUS: GlobalSignal<Option<BirdPack>> = Signal::global(|| None);

#[derive(Clone, Copy)]
pub struct AppCtx {
    pub auth_state: AuthState,
    pub stats: Sync<Stats>,
}

impl AppCtx {
    pub fn init() {
        let auth_state = AuthState::init();
        let stats = Sync::<Stats>::init(auth_state);
        use_context_provider(|| Self { auth_state, stats });
    }
}

// TODO: add google fonts via asset! (see docs)
pub fn App() -> Element {
    AppCtx::init();
    const ARCADE_FONT: &str = asset!(font().families(["Bungee Shade"]));
    const MONO_FONT: &str = asset!(font().families(["Space Mono"]).weights([400, 700]));
    rsx! {
        head::Link {
            rel: "stylesheet",
            href: asset!("assets/tailwind.css"),
        }
        head::Link { rel: "stylesheet",  href: ARCADE_FONT }
        head::Link { rel: "stylesheet",  href: MONO_FONT }
        Router::<Route> {
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(HeaderFooter)]
        // #[route("/login/#:fragment")]
        // LoginRedirect {
        //     fragment: MagicLinkResponse
        // },

        #[route("/")]
        Index {},

        #[route("/play/:pack_id")]
        Play {
            pack_id: u64
        },

        #[route("/birds")]
        Birds {},
}

#[component]
fn HeaderFooter() -> Element {
    rsx! {
        div {
            class: "flex flex-col sm:h-dvh pb-2",
            header {
                id: "header",
                class: "text-green-dark shrink h-20 py-2 w-full flex flex-row justify-between items-center",
                div {}
                div {
                    class: "text-5xl font-arcade font-semibold uppercase",
                    "birdtalk"
                }
                div {}
            }
            div {
                id: "content",
                class: "no-shrink",
                Outlet::<Route> {
                }
            }
            footer {
                id: "footer",
                class: "shrink sticky top-[100vh] hidden sm:flex justify-items-center justify-center",
                div {
                    "© 2024 birdtalk"
                }
            }
        }
    }
}
