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
    ui::pages::{Birds, Index, Play},
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
    let route: Route = use_route();
    const AVIARY: ImageAsset = asset!(image("assets/aviary.png").size(80, 96));
    const AVIARY_ACTIVE: ImageAsset = asset!(image("assets/aviary_active.png").size(80, 96));
    let aviary_src = if matches!(route, Route::Birds {}) {
        AVIARY_ACTIVE
    } else {
        AVIARY
    };
    let aviary_src = aviary_src.strip_prefix("./assets").unwrap_or(&aviary_src);
    rsx! {
        div {
            class: "flex flex-col sm:h-dvh selection:bg-purple-dark overflow-x-clip sm:overflow-x-visible",
            header {
                id: "header",
                class: "text-green-dark grow-0 shrink-0 px-1 py-2 w-full flex flex-row justify-between sm:justify-center items-center gap-8",
                class: "h-16 sm:h-20 text-4xl sm:text-5xl",
                div {
                    class: "shrink-0",
                    Link {
                        class: "outline-none focus-visible:ring",
                        to: Route::Birds {},
                        div {
                            class: "bg-contain bg-no-repeat bg-center",
                            class: "h-10 w-8 sm:h-12 sm:w-10",
                            background_image: "url({aviary_src})",
                        }
                        span { class: "sr-only", "Your Aviary" }
                    }
                }
                div {
                    class: "font-arcade font-semibold uppercase",
                    h1 {
                        Link {
                            class: "outline-none focus-visible:ring",
                            to: Route::Index {}, "birdtalk"
                        }
                    }
                }
                // Just jank until another icon is here
                div {
                    class: "shrink-0 h-10 w-8 sm:h-12 sm:w-10",
                }
            }
            div {
                id: "content",
                class: "grow shrink-0",
                Outlet::<Route> {
                }
            }
            footer {
                id: "footer",
                class: "h-6 shrink grow-0 mt-auto hidden sm:flex justify-items-center justify-center",
                div {
                    "© 2024 birdtalk"
                }
            }
        }
    }
}
