use dioxus::prelude::*;

use crate::ui::Route;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            id: "header",
            class: "text-green-dark grow-0 shrink-0 px-1 py-2 w-full flex flex-row justify-between sm:justify-center items-center gap-8",
            class: "h-16 sm:h-20 text-4xl sm:text-5xl",
            div {
                class: "shrink-0",
                AviaryLink {}
            }
            div {
                class: "font-arcade font-semibold uppercase",
                h1 {
                    Link {
                        class: "outline-none focus-visible:ring",
                        to: Route::Index {}, "birdtalk"
                    }
                }
            }
            // Just jank until another icon is here
            div {
                class: "shrink-0",
                DonateLink {}
            }
        }
    }
}

#[component]
fn AviaryLink() -> Element {
    const AVIARY: ImageAsset = asset!(image("assets/aviary.png").size(80, 96));
    const AVIARY_ACTIVE: ImageAsset = asset!(image("assets/aviary_active.png").size(80, 96));
    let aviary_src = fix_asset_for_ssg(&AVIARY);
    let aviary_active_src = fix_asset_for_ssg(&AVIARY_ACTIVE);

    rsx! {
        Link {
            id: "aviary-header-link",
            class: "outline-none focus-visible:ring",
            active_class: "aviary-active",
            to: Route::Birds {},
            div {
                id: "aviary-header-img",
                class: "bg-contain bg-no-repeat bg-center",
                class: "h-10 w-8 sm:h-12 sm:w-10",
            }
            span { class: "sr-only", "Your Aviary" }
        }
        style {
            dangerous_inner_html: r#"
            #aviary-header-img {{
                background-image: url({aviary_src});
            }}
            #aviary-header-link:hover > #aviary-header-img,
            #aviary-header-link.aviary-active > #aviary-header-img {{
                background-image: url({aviary_active_src});
            }}
            "#
        }
    }
}

#[component]
fn DonateLink() -> Element {
    const DONATE: ImageAsset = asset!(image("assets/donate.png").size(80, 96));
    const DONATE_ACTIVE: ImageAsset = asset!(image("assets/donate_active.png").size(80, 96));
    let donate_src = fix_asset_for_ssg(&DONATE);
    let donate_active_src = fix_asset_for_ssg(&DONATE_ACTIVE);

    rsx! {
        Link {
            id: "donate-header-link",
            class: "outline-none focus-visible:ring",
            new_tab: true,
            to: "https://act.audubon.org/a/donate",
            div {
                id: "donate-header-img",
                class: "bg-contain bg-no-repeat bg-center",
                class: "h-10 w-8 sm:h-12 sm:w-10",
            }
            span { class: "sr-only", "Donate" }
        }
        style {
            dangerous_inner_html: r#"
            #donate-header-img {{
                background-image: url({donate_src});
            }}
            #donate-header-link:hover > #donate-header-img {{
                background-image: url({donate_active_src});
            }}
            "#
        }
    }
}

// SSG screws up asset file paths
fn fix_asset_for_ssg(asset: &ImageAsset) -> &str {
    asset.strip_prefix("./assets").unwrap_or(asset)
}
