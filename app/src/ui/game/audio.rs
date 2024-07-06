use dioxus::prelude::*;

use crate::{
    bird::BirdDetailed,
    ui::{AUDIO_AUTOPLAY, AUDIO_LOOP},
};

#[component]
pub fn AudioPlayer(bird: Memo<BirdDetailed>) -> Element {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlAudioElement;

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
        button {
            class: "rounded-full focus:outline-none focus-visible:ring focus-visible:ring-green-500",
            onclick: move |_| async move {
                if let Some(audio) = audio_element.read().as_ref() {
                    tracing::debug!("audio_element.src(): {:?}", audio.current_src());
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
                view_box: "0 0 24 24",
                fill: "none",
                stroke_width: "1.5",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                // TODO: gate landscape styling to mobile builds only.
                class: "text-green-800 w-16 h-16 sm:w-24 sm:h-24 sm:max-lg:landscape:w-16 sm:max-lg:landscape:h-16",
                path {
                    stroke_linejoin: "round",
                    stroke_linecap: "round",
                    d: if playing() {
                        "M14.25 9v6m-4.5 0V9M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                    } else {
                        "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z
                                                         M15.91 11.672a.375.375 0 0 1 0 .656l-5.603 3.113a.375.375 0 0 1-.557-.328V8.887c0-.286.307-.466.557-.327l5.603 3.112Z"
                    }
                }
            }
        }
        audio {
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
