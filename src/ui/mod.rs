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
                class: "grow-0 flex flex-col items-center bg-green-800 text-amber-50 py-2",
                img {
                    class: "w-24 mt-[-1rem] hidden sm:block",
                    src: "static_logo_transparent.png"
                }
                nav {
                    id: "navbar",
                    // TODO: padding arond the elements rather than container will give a bigger hitbox for the icons on mobile
                    class: "flex sm:flex-col gap-2 w-full justify-between sm:justify-start px-2 py-1 sm:py-4",
                    Link {
                        class: "flex items-center hover:text-amber-100 hover:bg-green-700 sm:px-4 sm:py-2",
                        to: Route::Learn {},
                        svg {
                            class: "w-8 h-8 sm:w-6 sm:h-6 sm:mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                stroke_linejoin: "round",
                                d:"M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" ,
                                stroke_linecap: "round"
                            }
                        }
                        span {class: "hidden sm:inline", "Learn"}
                    }
                    Link {
                        class: "flex items-center hover:text-amber-100 hover:bg-green-700 sm:px-4 sm:py-2",
                        to: Route::Listen {},
                        svg {
                            class: "w-8 h-8 sm:w-6 sm:h-6 sm:mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                stroke_linejoin: "round",
                                d:"m9 9 10.5-3m0 6.553v3.75a2.25 2.25 0 0 1-1.632 2.163l-1.32.377a1.803 1.803 0 1 1-.99-3.467l2.31-.66a2.25 2.25 0 0 0 1.632-2.163Zm0 0V2.25L9 5.25v10.303m0 0v3.75a2.25 2.25 0 0 1-1.632 2.163l-1.32.377a1.803 1.803 0 0 1-.99-3.467l2.31-.66A2.25 2.25 0 0 0 9 15.553Z" ,
                                stroke_linecap: "round"
                            }
                        }
                        span {class: "hidden sm:inline", "Listen"}
                    }
                    Link {
                        class: "flex items-center hover:text-amber-100 hover:bg-green-700 sm:px-4 sm:py-2",
                        to: Route::Birds {},
                        svg {
                            class: "w-8 h-8 sm:w-6 sm:h-6 sm:mr-2",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            stroke_width: "1.5", // 2
                            xmlns: "http://www.w3.org/2000/svg",
                            stroke_linejoin: "round",
                            stroke_linecap: "round",
                            path { d: "M16 7h.01" }
                            path { d: "M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20" }
                            path { d: "m20 7 2 .5-2 .5" }
                            path { d: "M10 18v3" }
                            path { d: "M14 17.75V21" }
                            path { d: "M7 18a6 6 0 0 0 3.84-10.61" }
                        }
                        span {class: "hidden sm:inline", "Birds"}
                    }
                    Link {
                        class: "hidden sm:flex items-center hover:text-amber-100 hover:bg-green-700 sm:px-4 sm:py-2",
                        to: Route::Birds {},
                        svg {
                            class: "w-8 h-8 sm:w-6 sm:h-6 sm:mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                stroke_linejoin: "round",
                                d:"M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3" ,
                                stroke_linecap: "round"
                            }
                        }
                        span {class: "hidden sm:inline", "Packs"}
                    }
                    Link {
                        class: "flex items-center hover:text-amber-100 hover:bg-green-700 sm:px-4 sm:py-2",
                        to: Route::Achievements {},
                        svg {
                            class: "w-8 h-8 sm:w-6 sm:h-6 sm:mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                stroke_linejoin: "round",
                                d:"M16.5 18.75h-9m9 0a3 3 0 0 1 3 3h-15a3 3 0 0 1 3-3m9 0v-3.375c0-.621-.503-1.125-1.125-1.125h-.871M7.5 18.75v-3.375c0-.621.504-1.125 1.125-1.125h.872m5.007 0H9.497m5.007 0a7.454 7.454 0 0 1-.982-3.172M9.497 14.25a7.454 7.454 0 0 0 .981-3.172M5.25 4.236c-.982.143-1.954.317-2.916.52A6.003 6.003 0 0 0 7.73 9.728M5.25 4.236V4.5c0 2.108.966 3.99 2.48 5.228M5.25 4.236V2.721C7.456 2.41 9.71 2.25 12 2.25c2.291 0 4.545.16 6.75.47v1.516M7.73 9.728a6.726 6.726 0 0 0 2.748 1.35m8.272-6.842V4.5c0 2.108-.966 3.99-2.48 5.228m2.48-5.492a46.32 46.32 0 0 1 2.916.52 6.003 6.003 0 0 1-5.395 4.972m0 0a6.726 6.726 0 0 1-2.749 1.35m0 0a6.772 6.772 0 0 1-3.044 0" ,
                                stroke_linecap: "round"
                            }
                        }
                        span {class: "hidden sm:inline", "Achievements"}
                    }
                    Link {
                        class: "flex items-center hover:text-amber-100 hover:bg-green-700 sm:px-4 sm:py-2",
                        to: Route::Settings {},
                        svg {
                            class: "w-8 h-8 sm:w-6 sm:h-6 sm:mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                stroke_linejoin: "round",
                                d: "M10.5 6h9.75M10.5 6a1.5 1.5 0 1 1-3 0m3 0a1.5 1.5 0 1 0-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-9.75 0h9.75",
                                stroke_linecap: "round"
                            }
                        }
                        span {class: "hidden sm:inline", "Settings"}
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
