use dioxus::prelude::*;

use crate::{
    bird::{Bird, BirdPackDaily},
    ui::{components::icons::ArrowUturnRightIcon, Route, PLAY_STATUS},
};

/// Pack of the day
#[component]
pub fn PackOfTheDay() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |errors: ErrorContext| rsx! {
                div {
                    class: "text-red-dark",
                    div { "Drats! Something went wrong fetching today's challenge. Please open a GitHub issue with the following error:" }
                    code { "{errors:?}" }
                }
            },
            SuspenseBoundary {
                fallback: |_context: SuspenseContext| rsx! {
                    div {"some pretty pulsing loading pack!"}
                },
                PackOfTheDayInner {}
            }
        }
    }
}

// TODO: "Daily Bevy" in a half circle as title above?
// TODO: arrow key shortcuts
// TODO: "Next pack in ..."
#[component]
fn PackOfTheDayInner() -> Element {
    let BirdPackDaily { pack, day: _ } = use_resource(BirdPackDaily::fetch_today)
        .suspend()?
        .read()
        .clone()?;
    let pack_size = pack.birds.len();
    let mut position = use_signal(|| 0usize);
    let playing = use_signal(|| false);

    rsx! {
        div {
            class: "grid grid-cols-5 items-center mx-auto overflow-x-clip sm:overflow-x-visible",
            button {
                class: "col-span-1 w-12 h-12 focus:outline-none focus-visible:ring focus-visible:ring-black font-semibold bg-offwhite text-black border-2 rounded-full shadow sm:hover:shadow-xl sm:hover:scale-110 flex justify-center items-center z-40 justify-self-end sm:justify-self-center order-last sm:order-first",
                onclick: move |_| {
                    position.with_mut(|p| *p = (*p + 1) % pack_size);
                },
                ArrowUturnRightIcon {}
            }
            div {
                class: "col-start-2 col-span-3 justify-self-stretch flex flex-col gap-6 items-center justify-center",
                ul {
                    class: "w-56 h-96 relative",
                    for (ix, bird) in pack.birds.clone().into_iter().enumerate() {
                        Card { bird, playing, ix, pack_size, position }
                    }
                }
                button {
                    class: "px-12 py-4 mt-2 border-2 border-green-extra-dark focus:outline-none focus-visible:ring focus-visible:ring-green-dark font-semibold text-base bg-green-dark text-white rounded-xl shadow sm:hover:shadow-xl sm:hover:scale-125 sm:hover:bg-gradient-to-r from-green to-green-dark transition-transform uppercase text-xl z-40",
                    onclick: move |_| {
                        *PLAY_STATUS.write() = Some(pack.clone());
                        navigator().push(Route::Play { pack_id: pack.id });
                    },
                    "play"
                }
            }
        }
    }
}

/// Render the individual cards in the daily pack component.
/// - `playing` is the status of audio, which we maintain across cards
/// - `ix` is the index of the bird in the pack, which does not change.
/// - `pack_size` is the total number of birds in the pack.
/// - `position` is the index of the card from the user's perspective.
///    This changes when the user clicks to view the next card.
#[component]
fn Card(
    bird: Bird,
    playing: Signal<bool>,
    ix: usize,
    pack_size: usize,
    position: Signal<usize>,
) -> Element {
    let degree = |pos: usize| match pos {
        0 => 0,
        1 => 5,
        2 => 8,
        3 => 10,
        i => i + 7,
    };
    let bg_color = |ix: usize| match ix % 8 {
        0 => "bg-green",
        1 => "bg-yellow",
        2 => "bg-blue-light",
        3 => "bg-orange",
        4 => "bg-purple",
        5 => "bg-red",
        6 => "bg-chartreuse",
        7 => "bg-pink",
        _ => unreachable!(),
    };
    let pos = use_memo(move || (ix + pack_size - position()) % pack_size);
    let visible = use_memo(move || pos() == 0);
    rsx! {
        li {
            key: ix,
            class: "absolute inset-0 border rounded-xl shadow py-3 sm:py-4 text-black {bg_color(ix)} flex flex-row justify-between transition-transform transform-gpu duration-700 origin-bottom select-none",
            // NOTE: this overwrites transform-gpu :/ I could make another closure
            // to compute hardcoded transform strings, so that its tailwind all the way down.
            transform: "rotate({degree(pos())}deg) translateX({degree(pos())}px)",
            z_index: "{pack_size - pos()}",
            "data-position": "{pos()}",

            // current
            class: if pos() == 0 {
                "select-text will-change-transform"
            },

            // next
            class: if pos() == 1 {
                "z"
            },

            // last
            class: if pos() == pack_size - 1 {
                "animate-card-slide-out"
            },

            div {
                class: "uppercase max-h-full self-end whitespace-nowrap text-ellipsis overflow-hidden",
                text_orientation: "upright",
                writing_mode: "vertical-lr",
                "{bird.scientific_name.split_whitespace().next().unwrap()}"
            }

            // center
            div {
                class: "flex flex-col gap-4 items-center",
                img {
                    class: "border-2 w-24 h-24 rounded-full object-cover flex-none overflow-hidden",
                    src: bird.image_url(),
                    alt: "{bird.common_name}",
                }
                div {
                    class: "text-lg text-center select-all",
                    "{bird.common_name}"
                }
                div {
                    class: "mt-auto mb-8",
                    Audio { url: bird.default_sound_url(), user_playing: playing, visible }
                }
            }

            div {
                class: "uppercase max-h-full self-start whitespace-nowrap text-ellipsis overflow-hidden",
                text_orientation: "upright",
                writing_mode: "vertical-lr",
                "{bird.scientific_name.split_whitespace().last().unwrap()}"
            }
        }
    }
}

/// When the user clicks on the play button on a card, we start playing its audio.
/// If the user clicks to view the next card, the functionality is like a "skip"; that is, we keep
/// playing audio but skip to the next bird.
/// Thus for the individual audio elements on each card, we need to know both the user
/// audio-playing & card-visible status.
///
/// We use effects to change the play status on changes to these signals, rather than the signals
/// themselves. This is to ensure that a screen reader user (for which "visible" doesn't mean
/// anything here) can still use any card's audio button at any time.
#[component]
pub fn Audio(url: String, user_playing: Signal<bool>, visible: ReadOnlySignal<bool>) -> Element {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlAudioElement;

    let mut audio_element: Signal<Option<HtmlAudioElement>> = use_signal(|| None);
    let mut this_playing = use_signal(|| false);

    use_effect(move || {
        // When this card becomes invisible its audio is playing, pause it after 750 ms.
        // The effect is that the card keeps playing until the animation to put it at the back of
        // the deck is finished.
        if this_playing() && !visible() {
            spawn(async move {
                #[cfg(feature = "web")]
                async_std::task::sleep(std::time::Duration::from_millis(750)).await;
                if let Some(audio) = audio_element.read().as_ref() {
                    audio.pause().ok();
                }
            });
        }
        // When this card becomes visible and the user is already playing audio, don't delay, just
        // start playing this card. There's a small but pleasant overlap in audio.
        if user_playing() && visible() {
            spawn(async move {
                if let Some(audio) = audio_element.read().as_ref() {
                    if let Ok(promise) = audio.play() {
                        wasm_bindgen_futures::JsFuture::from(promise).await.ok();
                    }
                }
            });
        }
    });

    rsx! {
        button {
            class: "border-2 p-2 rounded-full focus:outline-none focus-visible:ring focus-visible:ring-black text-black/80",
            disabled: !visible(),
            onclick: move |_| async move {
                if let Some(audio) = audio_element.read().as_ref() {
                    if audio.paused() {
                        if let Ok(promise) = audio.play() {
                            wasm_bindgen_futures::JsFuture::from(promise).await.ok();
                            user_playing.set(true);
                        }
                    } else {
                        audio.pause().ok();
                        user_playing.set(false);
                    }
                }
            },
            if this_playing() {
                PauseIcon {}
            } else {
                SoundIcon {}
            }
        }
        audio {
            onmounted: move |mnt| {
                audio_element
                    .set(
                        mnt.downcast::<web_sys::Element>().cloned().map(|el| el.unchecked_into()),
                    )
            },
            onplay: move |_| *this_playing.write() = true,
            onpause: move |_| *this_playing.write() = false,
            // controls: "true",
            preload: "auto",
            r#loop: true,
            autoplay: false,
            source {
                r#type: "audio/mpeg",
                src: url
            }
            "Your browser does not support the audio element."
        }
    }
}

#[component]
fn SoundIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "w-8 h-8",
            path {
                d: "M13.5 4.06c0-1.336-1.616-2.005-2.56-1.06l-4.5 4.5H4.508c-1.141 0-2.318.664-2.66 1.905A9.76 9.76 0 0 0 1.5 12c0 .898.121 1.768.35 2.595.341 1.24 1.518 1.905 2.659 1.905h1.93l4.5 4.5c.945.945 2.561.276 2.561-1.06V4.06ZM18.584 5.106a.75.75 0 0 1 1.06 0c3.808 3.807 3.808 9.98 0 13.788a.75.75 0 0 1-1.06-1.06 8.25 8.25 0 0 0 0-11.668.75.75 0 0 1 0-1.06Z"
            }
            path {
                d: "M15.932 7.757a.75.75 0 0 1 1.061 0 6 6 0 0 1 0 8.486.75.75 0 0 1-1.06-1.061 4.5 4.5 0 0 0 0-6.364.75.75 0 0 1 0-1.06Z"
            }
        }
    }
}

#[component]
fn PauseIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "w-8 h-8",
            path {
                fill_rule: "evenodd",
                d: "M6.75 5.25a.75.75 0 0 1 .75-.75H9a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H7.5a.75.75 0 0 1-.75-.75V5.25Zm7.5 0A.75.75 0 0 1 15 4.5h1.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H15a.75.75 0 0 1-.75-.75V5.25Z",
                clip_rule: "evenodd"
            }
        }
    }
}
