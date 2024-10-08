//! In [`crate::ui`] we have all of the actual markup that makes up the app. I.e. usage of rsx!
//! lives here.

mod components;
mod game;
mod pages;

use dioxus::prelude::*;

use crate::{
    pack::PackIdentifier,
    stats::Stats,
    supabase::AuthState,
    sync::Sync,
    ui::{
        components::{icons, Header},
        pages::{Birds, Index, Play},
    },
};

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

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(HeaderFooter)]
        // #[route("/login/#:fragment")]
        // LoginRedirect {
        //     fragment: MagicLinkResponse
        // },

        #[route("/")]
        Index {},

        #[route("/play/?:..pack_id")]
        Play {
            pack_id: PackIdentifier,
        },

        #[route("/birds")]
        Birds {},
}

#[component]
fn HeaderFooter() -> Element {
    rsx! {
        div {
            class: "flex flex-col sm:h-dvh selection:bg-purple-dark overflow-x-clip sm:overflow-x-visible",
            Header {}
            div {
                id: "content",
                class: "grow shrink-0",
                Outlet::<Route> {
                }
            }
            footer {
                id: "footer",
                class: "h-6 shrink grow-0 mt-auto hidden sm:flex justify-center items-center",
                div {
                    "© 2024 birdtalk"
                }
                Link {
                    to: "https://github.com/samtay/birdtalk",
                    new_tab: true,
                    class: "ml-2",
                    icons::Github {}
                    span { class: "sr-only", "GitHub Repository" }
                }
            }
        }
    }
}
