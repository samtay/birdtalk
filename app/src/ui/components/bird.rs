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

            // left
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

#[derive(PartialEq, Props, Clone)]
pub struct BirdCardPlaceholderProps {
    extra_classes: Option<String>,
    extra_scientific_first_class: Option<String>,
    extra_scientific_second_class: Option<String>,
    #[props(default = true)]
    responsive: bool,
    children: Element,
}

#[component]
pub fn BirdCardPlaceholder(props: BirdCardPlaceholderProps) -> Element {
    let BirdCardPlaceholderProps {
        extra_classes,
        responsive,
        children,
        extra_scientific_first_class,
        extra_scientific_second_class,
    } = props;
    let extra_classes = extra_classes.unwrap_or_default();
    let extra_scientific_first_class = extra_scientific_first_class.unwrap_or_default();
    let extra_scientific_second_class = extra_scientific_second_class.unwrap_or_default();
    rsx! {
        div {
            class: "bg-offwhite-2 border border-black/10 rounded-xl py-3 sm:py-4 flex flex-row justify-between {extra_classes}",

            // left
            div {
                class: "ml-2 w-2 h-32 self-end bg-black/10 rounded-full {extra_scientific_first_class}",
                class: if responsive {
                    "hidden sm:block"
                },
            }

            // center
            div {
                class: "flex items-center",
                class: if responsive {
                    "flex-row sm:flex-col gap-1 sm:gap-4 px-1 sm:px-0 w-full"
                } else {
                    "flex-col gap-4"
                },
                div { class: "w-24 h-24 rounded-full flex-none bg-black/10" }
                div {
                    class: "flex flex-col justify-center items-center gap-4 mx-auto",
                    div {
                        class: "h-2.5 bg-black/20 rounded-full",
                        class: if responsive {
                            "w-40 sm:w-24"
                        } else {
                            "w-24"
                        }
                    }
                    div {
                        class: "h-2.5 bg-black/20 rounded-full",
                        class: if responsive {
                            "w-36 sm:w-28"
                        } else {
                            "w-28"
                        }
                    }
                }
                {children}
            }

            // right
            div {
                class: "mr-2 w-2 {extra_scientific_second_class} self-start bg-black/10 rounded-full",
                class: if responsive {
                    "hidden sm:block"
                }
            }
        }
    }
}
