use dioxus::prelude::*;

use crate::bird::BirdPackDetailed;

#[component]
pub fn BirdPack(pack: BirdPackDetailed, selection: Signal<Option<BirdPackDetailed>>) -> Element {
    rsx! {
        li {
            label {
                r#for: pack.id as i64,
                class: "sm:flex-col gap-4 justify-between inline-flex h-full w-full border-2 rounded-xl shadow p-3 sm:p-4 transition-transform has-[:enabled]:hover:-translate-y-2 has-[:enabled]:hover:bg-yellow-light has-[:enabled]:hover:shadow-xl has-[:disabled]:opacity-50 focus-within:ring-2 focus-within:ring-purple-dark has-[:checked]:bg-purple-light has-[:checked]:has-[:enabled]:hover:bg-purple-light has-[:checked]:text-black cursor-pointer select-none relative",
                input {
                    class: "absolute opacity-0 peer",
                    name: "pack",
                    id: pack.id as i64,
                    value: pack.id as i64,
                    r#type: "radio",
                    checked: selection.as_ref().filter(|bp| bp.id == pack.id).map(|_|true),
                    // TODO: onmount should probably be replaced with use_effect if its not using mount data?
                    // TODO: join with user data to send packs down with a "default choice" depending on where a user is at
                    onmounted: {
                        let pack = pack.clone();
                        move |_| {
                            if pack.name == "Common I" {
                                tracing::debug!("onmount: setting pack to {:?}", pack.id);
                                *selection.write() = Some(pack.clone())
                            }
                        }
                    },
                    onchange: {
                        let pack = pack.clone();
                        move |_| {
                            tracing::debug!("onchange: setting pack to {:?}", pack.id);
                            *selection.write() = Some(pack.clone());
                    }}
                }
                svg {
                    class: "w-6 h-6 text-purple-dark inline-block absolute right-2 top-2 invisible sm:peer-checked:visible",
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
                    class: "w-6 h-6 text-black inline-block absolute right-2 top-2 invisible sm:visible peer-checked:invisible",
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
                                span {class: "max-sm:w-8 max-sm:h-8 sm:w-9 sm:h-9 rounded-full flex-none bg-purple"}
                            }
                        } else {
                            for bird in pack.birds.iter().take(10) {
                                // TODO: tooltip with common name
                                img {
                                    class: "max-sm:w-8 max-sm:h-8 sm:w-9 sm:h-9 rounded-full object-cover flex-none max-sm:min-w-8 max-sm:min-h-8 sm:min-w-9 sm:min-h-9 overflow-hidden",
                                    src: bird.image_url(),
                                    alt: bird.common_name.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
