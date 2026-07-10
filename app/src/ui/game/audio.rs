use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlAudioElement;

use super::spectrogram::Spectrogram;
use crate::bird::Bird;

const AUDIO_LOOP: bool = true;
const AUDIO_AUTOPLAY: bool = true;

#[component]
pub fn AudioPlayer(bird: Memo<Bird>) -> Element {
    let mut audio_element: Signal<Option<HtmlAudioElement>> = use_signal(|| None);
    let mut playing: Signal<bool> = use_signal(|| false);

    // Explicitly audio.load() on changes to bird, otherwise the first audio element gets persisted
    // indefinitely.
    use_effect(move || {
        let _ = bird.read();
        if let Some(audio) = audio_element.read().as_ref() {
            audio.load();
        }
    });

    rsx! {
        div {
            class: "flex items-center gap-2 sm:gap-3 w-full max-w-xs mx-auto p-1.5 sm:p-2 rounded-xl border border-black/10 bg-offwhite-1 shadow",
            button {
                class: "flex-shrink-0 flex items-center justify-center w-11 h-11 sm:w-14 sm:h-14 rounded-full bg-pink-dark focus:outline-none focus-visible:ring-2 sm:hover:scale-110 transition-transform",
                onclick: move |_| async move {
                    if let Some(audio) = audio_element.read().as_ref() {
                        tracing::trace!("audio_element.src(): {:?}", audio.current_src());
                        if audio.paused() {
                            if let Ok(promise) = audio.play() {
                                wasm_bindgen_futures::JsFuture::from(promise).await.ok();
                            }
                        } else {
                            audio.pause().ok();
                        }
                    }
                },
                svg {
                    class: "text-white w-5 h-5 sm:w-6 sm:h-6",
                    view_box: "0 0 24 24",
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                    "aria-hidden": "true",
                    "focusable": "false",
                    path {
                        d: if playing() {
                            "M6 5h4v14H6zM14 5h4v14h-4z"
                        } else {
                            "M8 5v14l11-7z"
                        }
                    }
                }
                span {
                    class: "sr-only",
                    if playing() { "Pause" } else { "Play" }
                }
            }
            Spectrogram { bird, audio_element, playing }
            audio {
                "crossorigin": "anonymous",
                onmounted: move |mnt| {
                    audio_element
                        .set(
                            mnt.downcast::<web_sys::Element>().cloned().map(|el| el.unchecked_into()),
                        )
                },
                onplay: move |_| *playing.write() = true,
                onpause: move |_| *playing.write() = false,
                // controls: "true",
                preload: "auto",
                r#loop: AUDIO_LOOP,
                autoplay: AUDIO_AUTOPLAY,
                source {
                    r#type: "audio/mpeg",
                    src: bird.read().default_sound_url()
                }
                "Your browser does not support the audio element."
            }
        }
    }
}
