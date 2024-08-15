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
    ui::components::{
        BirdIcon, BirdPack, Login, LoginModal, LoginRedirect, MusicNoteIcon, NavbarLink, PacksIcon,
        PlayIcon, SettingsIcon, TrophyIcon,
    },
};
use game::GameView;
use pages::{Achievements, Birds, Listen, Packs, Settings};

const AUDIO_LOOP: bool = true;
const AUDIO_AUTOPLAY: bool = true;

static GAME_STATUS: GlobalSignal<GameStatus> = Signal::global(|| GameStatus::None);

#[derive(Debug, Clone, PartialEq)]
pub enum GameStatus {
    None,
    Playing(BirdPackDetailed),
    PlayingWithProgress(BirdPackDetailed),
}

impl GameStatus {
    pub fn playing(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn has_progress(&self) -> bool {
        matches!(self, Self::PlayingWithProgress(_))
    }

    pub fn pack(&self) -> Option<&BirdPackDetailed> {
        match self {
            Self::Playing(pack) => Some(pack),
            Self::PlayingWithProgress(pack) => Some(pack),
            _ => None,
        }
    }
}

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
        Router::<Route> {
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[route("/login/#:fragment")]
    LoginRedirect {
        fragment: MagicLinkResponse
    },

    // #[layout(Navbar)]
    // #[layout(LoginGate)]
    #[layout(HeaderFooter)]
    #[route("/")]
    Learn {},

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
    let on_open_route = matches!(use_route(), Route::Learn {});
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
                        to: Route::Learn {},
                        icon: rsx! {PlayIcon {}},
                        label: "Learn"
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
    rsx! {
        div {
            class: "flex flex-col sm:h-dvh pb-2 sm:max-lg:landscape:justify-center",
            header {
                id: "header",
                class: "text-chartreuse-light shrink container h-24 max-w-screen-md mt-2 mx-auto bg-contain bg-center bg-no-repeat",
                background_image: "url({HEADER})",
                class: if GAME_STATUS.read().playing() {
                    "hidden"
                },
            }
            div {
                id: "content",
                class: "no-shrink",
                Outlet::<Route> {
                }
            }
            footer {
                id: "footer",
                class: "shrink sticky top-[100vh] hidden sm:flex justify-items-center justify-center sm:max-lg:landscape:hidden",
                div {
                    "© 2024 birdtalk"
                }
            }
        }
    }
}

#[component]
fn Learn() -> Element {
    match &*GAME_STATUS.read() {
        GameStatus::None => {
            rsx! {
                GameSelector { }
            }
        }
        GameStatus::Playing(pack) | GameStatus::PlayingWithProgress(pack) => {
            rsx! {
                GameView {
                    pack: pack.clone(),
                }
            }
        }
    }
}

#[component]
fn GameSelector() -> Element {
    let selected_pack = use_signal(|| None);

    rsx! {
        div {
            class: "container max-w-screen-lg m-auto mt-2 px-2 landscape:max-lg:px-1 sm:px-4 flex flex-col items-stretch gap-6",
            PackSelector {selected_pack}
            button {
                class: "border border-black flex-none mt-2 px-4 py-2 focus:outline-none focus-visible:ring focus-visible:ring-chartreuse-dark font-semibold text-base border bg-chartreuse rounded-xl shadow enabled:hover:bg-chartreuse-dark",
                onclick: move |_| {
                    *GAME_STATUS.write() = GameStatus::Playing(selected_pack().unwrap());
                    tracing::debug!("GameStatus: {:?}", *GAME_STATUS.read())
                },
                disabled: selected_pack.read().is_none(),
                "Let's Go!"
            }
        }
    }
}

#[component]
fn PackSelector(selected_pack: Signal<Option<BirdPackDetailed>>) -> Element {
    let packs_resource = use_resource(BirdPackDetailed::fetch_free_packs);
    let packs_value = packs_resource.value();
    let packs_read = packs_value.read();
    match *packs_read {
        None => rsx! {PackSelectorPlaceholder {}},
        Some(Err(ref e)) => rsx! {"TODO handle errors gracefully: {e}"},
        Some(Ok(ref packs)) => rsx! {
            fieldset {
                legend {
                    class: "font-semibold text-2xl text-center mb-2",
                    "Pick a bird pack"
                }
                ul {
                    class: "grid grid-cols-1 sm:grid-cols-3 w-full gap-2 sm:gap-6 items-stretch",
                    for pack in packs.clone() {
                        BirdPack {pack, selection: selected_pack}
                    }
                }
            }
        },
    }
}

#[component]
fn PackSelectorPlaceholder() -> Element {
    rsx! { Loading { } }
}

#[component]
fn Loading() -> Element {
    rsx! {
        div {
            class: "mt-12 flex flex-col items-center justify-center",
            div {
                class: "animate-spin w-24 h-24 border-t-4 border-green-800 rounded-full"
            }
        }
    }
}
