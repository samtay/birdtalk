use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ModalProps {
    pub children: Element,
    pub extra_backdrop_classes: Option<String>,
    pub extra_modal_classes: Option<String>,
    /// If dismissable, an X button will be shown.
    /// If dismissable, clicking the X button or backdrop will close the modal.
    #[props(default = true)]
    pub dismissable: bool,
    pub on_dismiss: Option<UseCallback<()>>,
}

// NOTE: Moving this to a permanent location at a top level in the DOM would allow for us to swap
// the background content (e.g. go from game mode to home screen) while the backdrop is blurred.
// This might be nice; currently a bit abrupt.
pub fn Modal(props: ModalProps) -> Element {
    let ModalProps {
        children,
        extra_backdrop_classes,
        extra_modal_classes,
        dismissable,
        on_dismiss,
    } = props;

    let mut dismissed = use_signal(|| false);

    let mut on_dismiss_handler = move |_| {
        if dismissable {
            tracing::debug!("Setting dismissed = true");
            dismissed.set(true);
            if let Some(on_dismiss) = on_dismiss {
                tracing::debug!("Calling on_dismiss");
                on_dismiss.call();
            }
        }
    };

    rsx! {
        div {
            class: "fixed inset-0 z-20 flex justify-center items-center backdrop-blur-sm bg-amber-50/20 overflow-hidden transition-[opacity,backdrop-blur] duration-1000",
            class: if let Some(extra_backdrop_classes) = extra_backdrop_classes {
                extra_backdrop_classes
            },
            class: if dismissed() {
                "invisible z-[-1] opacity-0 backdrop-blur-none"
            },
            onclick: move |e| {
                tracing::debug!("Modal background pressed; dismissing!");
                on_dismiss_handler(e);
            },
            div {
                class: "w-full h-auto bottom-0 absolute bg-amber-50 absolute animate-slide-up sm:static sm:w-3/5 sm:h-auto sm:min-w-[640px] sm:rounded-lg",
                class: if let Some(extra_modal_classes) = extra_modal_classes {
                    extra_modal_classes
                },
                class: if dismissed() {
                    "animate-slide-down"
                },
                onclick: |e| {
                    // Don't close modal when interacting with its content
                    tracing::debug!("Modal content click event; stopping propagation!");
                    e.stop_propagation();
                },
                button {
                    class: "absolute top-0 right-0 p-2",
                    class: if dismissable {
                        "visible"
                    } else {
                        "invisible"
                    },
                    onclick: move |e| {
                        tracing::debug!("Modal button pressed; dismissing!");
                        on_dismiss_handler(e);
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke_width: "1.5",
                        class: "w-6 h-6",
                        path {
                            stroke_linejoin: "round",
                            d: "M6 18 18 6M6 6l12 12",
                            stroke_linecap: "round"
                        }
                    }

                }
                {children}
            }
        }
    }
}
