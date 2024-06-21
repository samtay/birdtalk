mod components;
mod db;
mod game;
mod pages;

use dioxus::prelude::*;
use dioxus_sdk::storage::use_singleton_persistent;

use crate::bird::BirdPack;
use game::{GameMode, GameView};
use pages::{Achievements, Birds, Listen, Settings};

const AUDIO_LOOP: bool = true;
const AUDIO_AUTOPLAY: bool = true;

const PACKS_CACHE_KEY: &str = "packs-cache";

static GAME_STATUS: GlobalSignal<GameStatus> = Signal::global(|| GameStatus::None);

#[derive(Debug, Clone, PartialEq)]
pub enum GameStatus {
    None,
    Playing(BirdPack, GameMode),
    // TODO just make this a boolean...
    PlayingWithProgress(BirdPack, GameMode),
}

impl GameStatus {
    pub fn playing(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }

    pub fn has_progress(&self) -> bool {
        match self {
            Self::PlayingWithProgress(_, _) => true,
            _ => false,
        }
    }

    pub fn pack(&self) -> Option<&BirdPack> {
        match self {
            Self::Playing(pack, _) => Some(pack),
            Self::PlayingWithProgress(pack, _) => Some(pack),
            _ => None,
        }
    }
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Learn {},

        #[route("/listen")]
        Listen {},

        #[route("/birds")]
        Birds {},

        // #[route("/packs")]
        // Packs {},

        #[route("/achievements")]
        Achievements {},

        #[route("/settings")]
        Settings {},
    // #[end_layout]
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            class: "flex sm:flex-row flex-col-reverse h-screen",
            div {
                class: "grow-0 bg-green-800 text-amber-50 p-2",
                img {
                    class: "w-24 mt-[-1rem] hidden sm:block",
                    src: "static_logo_transparent.png"
                }
                nav {
                    id: "navbar",
                    class: "flex sm:flex-col gap-2 justify-between sm:justify-start items-center",
                    Link {
                        class: "text-amber-50 hover:text-amber-100",
                        to: Route::Learn {}, "Learn"
                    }
                    Link {
                        class: "text-amber-50 hover:text-amber-100",
                        to: Route::Listen {}, "Listen"
                    }
                    Link {
                        class: "text-amber-50 hover:text-amber-100",
                        to: Route::Birds {}, "Birds"
                    }
                    // Link {
                    //     class: "text-amber-50 hover:text-amber-100",
                    //     to: Route::Packs {}, "Packs"
                    // }
                    Link {
                        class: "text-amber-50 hover:text-amber-100",
                        to: Route::Achievements {}, "Achievements"
                    }
                    Link {
                        class: "text-amber-50 hover:text-amber-100",
                        to: Route::Settings {}, "Settings"
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

// TODO: use this for landing page ( #[layout(HeaderFooter)] )
#[component]
fn HeaderFooter() -> Element {
    rsx! {
        div {
            class: "flex flex-col sm:h-dvh pb-2 sm:max-lg:landscape:justify-center",
            header {
                id: "header",
                class: "text-green-800 shrink container h-32 sm:h-48 md:h-64 w-full max-w-screen-md mt-2 sm:mt-4 mb-[-2rem] mx-auto sm:max-lg:landscape:hidden bg-[url('heading-2.gif')] bg-cover bg-center bg-no-repeat",
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
                    class: "text-green-800/75",
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
        GameStatus::Playing(pack, mode) | GameStatus::PlayingWithProgress(pack, mode) => {
            rsx! {
                GameView {
                    pack: pack.clone(),
                    mode: *mode
                }
            }
        }
    }
}

#[component]
fn GameSelector() -> Element {
    let birdpack = use_signal(|| None);
    let mode = use_singleton_persistent(GameMode::default);

    rsx! {
        div {
            class: "container max-w-screen-lg m-auto mt-2 px-2 landscape:max-lg:px-1 sm:px-4 flex flex-col items-stretch gap-6",
            PackSelector {birdpack}
            ModeSelector {mode}
            button {
                class: "mt-2 px-4 py-2 focus:outline-none focus-visible:ring focus-visible:ring-green-400 font-semibold text-base bg-green-800 text-amber-50 rounded-full shadow",
                onclick: move |_| {
                    *GAME_STATUS.write() = GameStatus::Playing(birdpack().unwrap(), mode());
                    tracing::debug!("GameStatus: {:?}", *GAME_STATUS.read())
                },
                disabled: birdpack.read().is_none(),
                "Let's Go!"
            }
        }
    }
}

#[component]
fn PackSelector(birdpack: Signal<Option<BirdPack>>) -> Element {
    // TODO: use_server_future to get options
    // let packs = use_server_future(get_packs_initial());
    // Temp shit
    let packs = [
        BirdPack {
            id: "review".to_string(),
            name: "Jog Your Memory".to_string(),
            description: "Review birds you've learned".to_string(),
            already_learned: false,
            birds: vec![],
        },
        BirdPack::demo(),
        BirdPack {
            id: "pack-of-the-day".to_string(),
            name: "Pack of the Day".to_string(),
            description: "The daily bevy!".to_string(),
            already_learned: false,
            birds: vec![],
        },
    ];

    rsx! {
        fieldset {
            legend {
                class: "font-semibold text-2xl text-center mb-2",
                "Pick a bird pack"
            }
            ul {
                class: "grid grid-cols-1 sm:grid-cols-3 w-full gap-2 sm:gap-6 items-stretch",
                {packs.iter().map(|pack| {
                    rsx! {
                        li {
                            label {
                                r#for: pack.id.as_str(),
                                class: "sm:flex-col gap-4 justify-between inline-flex h-full w-full bg-amber-50 border-2 border-amber-200 rounded-xl shadow p-3 sm:p-4 has-[:enabled]:hover:bg-amber-200 has-[:enabled]:hover:shadow-xl has-[:disabled]:opacity-50 focus-within:ring-2 focus-within:ring-green-400 has-[:checked]:border-green-400 has-[:checked]:bg-green-100/50 has-[:checked]:has-[:enabled]:hover:bg-green-100/50 has-[:checked]:text-green-800 cursor-pointer select-none relative",
                                input {
                                    class: "absolute opacity-0 peer",
                                    name: "pack",
                                    id: pack.id.as_str(),
                                    value: pack.id.as_str(),
                                    r#type: "radio",
                                    checked: birdpack.as_ref().filter(|bp| bp.id == pack.id).map(|_|true),
                                    disabled: pack.id != "demo",
                                    // TODO: onmount should probably be replaced with use_effect if its not using mount data?
                                    onmounted: {
                                        let pack = pack.clone();
                                        move |_| {
                                            if &pack.id == "demo" {
                                                tracing::debug!("onmount: setting pack to {:?}", pack.id());
                                                *birdpack.write() = Some(pack.clone())
                                            }
                                        }
                                    },
                                    onchange: {
                                        let pack = pack.clone();
                                        move |_| {
                                            tracing::debug!("onchange: setting pack to {:?}", pack.id());
                                            *birdpack.write() = Some(pack.clone());
                                    }}
                                }
                                svg {
                                    class: "w-6 h-6 text-green-400 inline-block absolute right-2 top-2 invisible sm:peer-checked:visible",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    stroke_width: "1.5",
                                    stroke: "currentColor",
                                    path {
                                        d: "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round"
                                    }

                                }
                                svg {
                                    class: "w-6 h-6 text-amber-950/50 inline-block absolute right-2 top-2 invisible sm:visible peer-checked:invisible",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    stroke_width: "1.5",
                                    stroke: "currentColor",
                                    path {
                                        d: "M9 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round"
                                    }

                                }
                                div {
                                    class: "text-lg font-semibold",
                                    "{pack.name}"
                                }
                                div {
                                    class: "flex-initial sm:flex-none overflow-hidden w-1/2 sm:w-full",
                                    div {
                                        class: "flex gap-1 sm:grid sm:grid-cols-5 sm:gap-2.5 sm:min-w-52",
                                        if pack.birds.is_empty() {
                                            for _ in 0..10 {
                                                span {class: "max-sm:w-8 max-sm:h-8 sm:w-9 sm:h-9 rounded-full flex-none bg-amber-900/20"}
                                            }
                                        } else {
                                            for bird in pack.birds.iter().take(10) {
                                                // TODO: tooltip with common name
                                                img {
                                                    class: "max-sm:w-8 max-sm:h-8 sm:w-9 sm:h-9 rounded-full object-cover flex-none max-sm:min-w-8 max-sm:min-h-8 sm:min-w-9 sm:min-h-9 overflow-hidden",
                                                    src: "{bird.img_file}",
                                                    alt: "{bird.common_name}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}

#[component]
fn ModeSelector(mode: Signal<GameMode>) -> Element {
    // I think checked only works with initial page load?
    rsx! {
        fieldset {
            legend {
                class: "font-semibold text-2xl text-center mb-2",
                "Pick a game mode"
            }
            ul {
                class: "grid grid-cols-1 sm:grid-cols-3 w-full gap-2 sm:gap-6 items-stretch",
                for opt in [GameMode::Listen, GameMode::Learn] {
                    li {
                        label {
                            r#for: "{opt}",
                            class: "flex-col inline-flex h-full w-full bg-amber-50 border-2 border-amber-200 rounded-xl shadow p-3 sm:p-4 has-[:enabled]:hover:bg-amber-200 has-[:enabled]:hover:shadow-xl has-[:disabled]:opacity-50 focus-within:ring-2 focus-within:ring-green-400 has-[:checked]:border-green-400 has-[:checked]:bg-green-100/50 has-[:checked]:has-[:enabled]:hover:bg-green-100/50 has-[:checked]:text-green-800 cursor-pointer select-none relative",
                            input {
                                class: "absolute opacity-0 peer",
                                name: "mode",
                                id: "{opt}",
                                value: "{opt}",
                                r#type: "radio",
                                checked: (*mode.read() == opt).then_some(true),
                                disabled: opt != GameMode::Learn,
                                onchange: move |_| {
                                    tracing::debug!("onchange: setting mode to from {:?} to {opt:?}", mode());
                                    *mode.write() = opt;
                                },
                            }
                            svg {
                                class: "w-6 h-6 text-green-400 inline-block absolute right-2 top-2 invisible sm:peer-checked:visible",
                                view_box: "0 0 24 24",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    d: "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round"
                                }

                            }
                            svg {
                                class: "w-6 h-6 text-amber-950/50 inline-block absolute right-2 top-2 invisible sm:visible peer-checked:invisible",
                                view_box: "0 0 24 24",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    d: "M9 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round"
                                }

                            }
                            p {
                                class: "text-lg font-semibold",
                                "{opt}"
                            }
                            p {
                                "{opt.description()}"
                            }
                            p {
                                class: "hidden sm:block mt-auto text-amber-950/60",
                                "{opt.pressure()}"
                            }
                        }
                    }
                }
            }
        }
    }
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
