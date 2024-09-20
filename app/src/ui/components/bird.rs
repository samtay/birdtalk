use dioxus::prelude::*;

use crate::bird::Bird;

#[derive(PartialEq, Props, Clone)]
pub struct BirdCardProps {
    extra_classes: Option<String>,
    #[props(default = true)]
    responsive: bool,
    bird: Bird,
    children: Element,
}

pub fn BirdCard(props: BirdCardProps) -> Element {
    let BirdCardProps {
        extra_classes,
        responsive,
        bird,
        children,
    } = props;
    let extra_classes = extra_classes.unwrap_or_default();
    rsx! {
        div {
            class: "flex flex-row justify-between border rounded-xl shadow py-3 sm:py-4 {extra_classes}",
            div {
                class: "uppercase max-h-full self-end whitespace-nowrap text-ellipsis overflow-hidden",
                class: if responsive {
                    "hidden sm:block"
                },
                text_orientation: "upright",
                writing_mode: "vertical-lr",
                "{bird.scientific_name.split_whitespace().next().unwrap()}"
            }

            // center
            div {
                class: "flex items-center",
                class: if responsive {
                    "flex-row sm:flex-col gap-1 sm:gap-4 px-1 sm:px-0 w-full"
                } else {
                    "flex-col gap-4"
                },
                img {
                    class: "border-2 w-24 h-24 rounded-full object-cover flex-none overflow-hidden",
                    src: bird.image_url(),
                    alt: "",
                }
                div {
                    class: "text-lg text-center select-all mx-auto",
                    "{bird.common_name}"
                }
                {children}
            }

            div {
                class: "uppercase max-h-full self-start whitespace-nowrap text-ellipsis overflow-hidden",
                class: if responsive {
                    "hidden sm:block"
                },
                text_orientation: "upright",
                writing_mode: "vertical-lr",
                "{bird.scientific_name.split_whitespace().last().unwrap()}"
            }
        }
    }
}
