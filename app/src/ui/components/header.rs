use dioxus::prelude::*;

use crate::ui::Route;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            id: "header",
            class: "text-green-dark grow-0 shrink-0 px-2 py-2 w-full flex flex-row justify-between sm:justify-center items-center sm:gap-8",
            class: "h-16 sm:h-20 text-4xl sm:text-5xl",
            div {
                class: "ml-1 shrink-0 rounded-xl has-[:focus-visible]:ring has-[:focus-visible]:ring-offset-4",
                AviaryLink {}
            }
            div {
                class: "font-arcade font-semibold uppercase",
                h1 {
                    Link {
                        class: "outline-none rounded-xl focus-visible:ring focus-visible:ring-offset-1",
                        to: Route::Index {}, "birdtalk"
                    }
                }
            }
            // Just jank until another icon is here
            div {
                class: "mr-1 shrink-0 rounded-xl has-[:focus-visible]:ring has-[:focus-visible]:ring-offset-4",
                DonateLink {}
            }
        }
    }
}

#[component]
fn AviaryLink() -> Element {
    const AVIARY: ImageAsset = asset!(image("assets/aviary_sprite.png"));
    let aviary_src = fix_asset_for_ssg(&AVIARY);

    rsx! {
        Link {
            id: "aviary-header-link",
            class: "outline-none",
            active_class: "aviary-active",
            to: Route::Birds {},
            div {
                id: "aviary-header-img",
                class: "h-10 w-8 sm:h-12 sm:w-10",
            }
            span { class: "sr-only", "Your Aviary" }
        }
        style {
            dangerous_inner_html: r#"
            #aviary-header-img {{
                background: no-repeat url({aviary_src}) 0 0 / 200% 100%;
            }}
            #aviary-header-link:hover > #aviary-header-img,
            #aviary-header-link.aviary-active > #aviary-header-img {{
                background-position: -40px -0px;
            }}
            "#
        }
    }
}

#[component]
fn DonateLink() -> Element {
    const DONATE: ImageAsset = asset!(image("assets/donate_sprite.png"));
    let donate_src = fix_asset_for_ssg(&DONATE);

    rsx! {
        Link {
            id: "donate-header-link",
            class: "outline-none",
            new_tab: true,
            to: "https://act.audubon.org/a/donate",
            div {
                id: "donate-header-img",
                class: "h-10 w-8 sm:h-12 sm:w-10",
            }
            span { class: "sr-only", "Donate" }
        }
        style {
            dangerous_inner_html: r#"
            #donate-header-img {{
                background: no-repeat url({donate_src}) 0 0 / 200% 100%;
            }}
            #donate-header-link:hover > #donate-header-img {{
                background-position: -40px -0px;
            }}
            "#
        }
    }
}

// SSG screws up asset file paths
fn fix_asset_for_ssg(asset: &ImageAsset) -> &str {
    asset.strip_prefix("./assets").unwrap_or(asset)
}
