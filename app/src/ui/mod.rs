mod components;
mod db;
mod game;
mod pages;

use dioxus::prelude::*;

use crate::{
    bird::BirdPackDetailed,
    stats::Stats,
    supabase::{AuthState, MagicLinkResponse},
    sync::Sync,
    ui::{
        components::{
            BirdIcon, Login, LoginModal, LoginRedirect, MusicNoteIcon, NavbarLink, PackOfTheDay,
            PacksIcon, PlayIcon, SettingsIcon, TrophyIcon,
        },
        game::GameViewPlaceholder,
    },
};
use game::GameView;
use pages::{Achievements, Birds, Listen, Packs, Settings};

const AUDIO_LOOP: bool = true;
const AUDIO_AUTOPLAY: bool = true;

pub static PLAY_STATUS: GlobalSignal<Option<BirdPackDetailed>> = Signal::global(|| None);

// TODO: this stuff will probably move. probably need user state to make db reqs
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
        script { src: "https://cdn.tailwindcss.com" }
        Router::<Route> {
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    // #[layout(LoginGate)]

    #[layout(HeaderFooter)]
        #[route("/login/#:fragment")]
        LoginRedirect {
            fragment: MagicLinkResponse
        },

        #[route("/")]
        Index {},

        #[route("/play/:pack_id")]
        Play {
            pack_id: u64
        },

        #[route("/listen")]
        Listen {},

        #[route("/birds")]
        Birds {},

        #[route("/packs")]
        Packs {},

        #[route("/achievements")]
        Achievements {},

        #[route("/settings")]
        Settings {},
}

#[component]
fn LoginGate() -> Element {
    let ctx = use_context::<AppCtx>();
    let on_open_route = matches!(use_route(), Route::Index {});
    let logged_in = use_memo(move || ctx.auth_state.is_logged_in());
    let login_needed = !on_open_route && !logged_in();
    // TODO: Perhaps arbitrarily delay to second generation() for SSG?
    //       Pending / refreshing -> Learn view shows placeholders
    //       Signed out           -> Fetch free packs with anon key
    //       Signed in            -> Fetch packs relevant to user
    rsx! {
        if login_needed {
            div {
                class: "flex flex-col items-center justify-center h-full",
                Login {}
            }
        } else {
            Outlet::<Route> { }
        }
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            class: "flex sm:flex-row flex-col-reverse h-screen",
            div {
                class: "grow-0 flex flex-col items-center sm:py-2",
                img {
                    class: "w-24 mt-[-1rem] hidden sm:block",
                    src: asset!("assets/static_logo_transparent.png")
                }
                nav {
                    id: "navbar",
                    // TODO: padding arond the elements rather than container will give a bigger hitbox for the icons on mobile
                    class: "flex sm:flex-col gap-2 w-full justify-between sm:justify-start sm:py-1 sm:py-4",
                    NavbarLink {
                        to: Route::Index {},
                        icon: rsx! {PlayIcon {}},
                        label: "Play"
                    }
                    NavbarLink {
                        to: Route::Listen {},
                        icon: rsx! {MusicNoteIcon {}},
                        label: "Listen"
                    }
                    NavbarLink {
                        to: Route::Birds {},
                        icon: rsx! {BirdIcon {}},
                        label: "Birds",
                    }
                    NavbarLink {
                        to: Route::Packs {},
                        icon: rsx! {PacksIcon {}},
                        label: "Packs",
                        desktop_only: true,
                    }
                    NavbarLink {
                        to: Route::Achievements {},
                        icon: rsx! {TrophyIcon {}},
                        label: "Achievements",
                    }
                    NavbarLink {
                        to: Route::Settings {},
                        icon: rsx! {SettingsIcon {}},
                        label: "Settings",
                    }
                }
            }
            div {
                id: "content",
                class: "grow no-shrink",
                Outlet::<Route> { }
            }
        }
    }
}

#[component]
fn HeaderFooter() -> Element {
    const HEADER: &str = manganis::mg!(file("assets/heading.webp"));
    // const HEADER: &str = manganis::mg!(file("assets/heading.gif")));
    // const COFFEE: ImageAsset = manganis::mg!(image("assets/coffee.png"));
    // const BIRDHOUSE: ImageAsset = manganis::mg!(image("assets/birdhouse.png"));
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

#[component]
fn Index() -> Element {
    rsx! {
        div {
            class: "container max-w-screen-lg m-auto mt-2 px-2 grid grid-cols-5 gap-5",
            div {
                class: "col-span-5 text-2xl text-center place-self-center",
                "Welcome to BirdTalk!"
            }
            div {
                class: "col-span-5 text-lg text-center place-self-center",
                "An app that helps you memorize bird calls"
            }
            div {
                class: "col-span-3 text-lg",
                "Try the Daily Bevy! Packs reset at midnight."
            }
            div {
                class: "col-span-2",
                PackOfTheDay { }
            }
        }
    }
}

#[component]
fn Play(pack_id: u64) -> Element {
    // Do I need reactivity on pack_id? https://docs.rs/dioxus-hooks/0.6.0-alpha.2/dioxus_hooks/fn.use_effect.html#with-non-reactive-dependencies

    // Typically PLAY_STATUS is already loaded with the proper birdpack
    let pack_to_play = use_memo(move || {
        PLAY_STATUS
            .read()
            .as_ref()
            .filter(|p| p.id == pack_id)
            .cloned()
    });

    // But if not (perhaps a fresh page load on this route),
    use_effect(move || {
        if pack_to_play.read().is_none() {
            spawn(async move {
                // TODO: how to handle errors here?
                let pack = BirdPackDetailed::fetch_by_id(pack_id).await.unwrap();
                *PLAY_STATUS.write() = Some(pack);
            });
        }
    });

    let x = match pack_to_play.read().as_ref() {
        Some(pack) => rsx! { GameView { pack: pack.clone() } },
        _ => rsx! { GameViewPlaceholder {}},
    };
    x
}
