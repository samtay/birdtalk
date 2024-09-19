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
    let route: Route = use_route();
    let is_index = matches!(route, Route::Index {});
    rsx! {
        div {
            class: "flex flex-col sm:h-dvh pb-2 selection:bg-purple-dark overflow-x-clip sm:overflow-x-visible",
            header {
                id: "header",
                class: "text-green-dark shrink py-2 w-full flex flex-row justify-between items-center",
                class: if is_index {
                    "h-20 text-5xl"
                } else {
                    "h-16 sm:h-20 text-4xl sm:text-5xl"
                },
                div {}
                div {
                    class: "font-arcade font-semibold uppercase",
                    h1 { Link { to: Route::Index {}, "birdtalk" } }
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
