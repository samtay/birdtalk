use dioxus::prelude::*;

#[component]
pub fn PlayIcon() -> Element {
    rsx! {
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
    }
}

#[component]
pub fn MusicNoteIcon() -> Element {
    rsx! {
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
    }
}

#[component]
pub fn BirdIcon() -> Element {
    rsx! {
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
    }
}

#[component]
pub fn PacksIcon() -> Element {
    rsx! {
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
    }
}

#[component]
pub fn TrophyIcon() -> Element {
    rsx! {
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
    }
}

#[component]
pub fn SettingsIcon() -> Element {
    rsx! {
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
    }
}
