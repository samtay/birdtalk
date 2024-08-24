mod components;
mod game;
mod pages;

use dioxus::prelude::*;

use crate::{
    bird::BirdPack,
    stats::Stats,
    supabase::{AuthState, MagicLinkResponse},
    sync::Sync,
    ui::{
        components::{
            BirdIcon, Login, LoginModal, LoginRedirect, MusicNoteIcon, PackOfTheDay, PacksIcon,
            PlayIcon, SettingsIcon, TrophyIcon,
        },
        game::{GameView, GameViewPlaceholder},
        pages::{Birds, Index, Play},
    },
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
    rsx! {
        head::Link {
            rel: "stylesheet",
            href: asset!("assets/tailwind.css"),
        }
        // TODO: remove
        // TODO: manganis?
        // script { src: "https://cdn.tailwindcss.com" }
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
    const HEADER: &str = manganis::mg!(file("assets/heading.webp"));
    rsx! {
        div {
            class: "flex flex-col sm:h-dvh pb-2",
            header {
                id: "header",
                class: "text-chartreuse-light shrink container h-24 max-w-screen-md mt-2 mx-auto bg-contain bg-center bg-no-repeat flex flex-row space-between items-center",
                background_image: "url({HEADER})",
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
