use dioxus::prelude::*;

use crate::{
    supabase::MagicLinkResponse,
    ui::{components::Modal, AppCtx, Route},
};

/// A modal wrapping the [`Login`] component.
#[component]
pub fn LoginModal() -> Element {
    rsx! {
        Modal {
            dismissable: false,
            div {
                class: "p-2 sm:p-4 mx-auto my-2",
                Login {}
            }
        }
    }
}

/// Log in component; currently supports magic link via email.
// TODO: basic email validation
// TODO: display 60s disabled before user can hit "signin" with different email
// TODO: allow annoymous signin
#[component]
pub fn Login() -> Element {
    let mut email = use_signal(String::new);
    let mut magic_link_sent = use_signal(|| false);

    let auth = use_context::<AppCtx>().auth_state;
    let signin = move |evt: FormEvent| async move {
        let values = evt.values();
        let email = values["email"][0].clone();
        tracing::info!("Logging in with email: {}", email);
        auth.sign_in_with_magic_link(email).await.unwrap();
        magic_link_sent.set(true);
    };

    rsx! {
        form { class: "text-center flex flex-col justify-center items-center gap-4 mb-5 w-full",
            onsubmit: signin,
            input {
                class: "w-64 px-3 py-2 rounded-lg border-none text-amber-50 text-green-800",
                "type": "email",
                name: "email",
                placeholder: "Enter your email",
                oninput: move |event| email.set(event.value())
            }
            button {
                class: "px-4 py-2 focus:outline-none focus-visible:ring focus-visible:ring-green-400 font-semibold text-base bg-green-800 text-amber-50 rounded-full shadow",
                disabled: magic_link_sent(),
                "Login / Signup"
            }
            if magic_link_sent() {
                div {
                    class: "text-sm text-green-400",
                    "We sent a magic link to your email! Check your inbox and click the link to login."
                }
            }
        }
    }
}

/// A component for receiving the magic link response and redirecting back to the main app.
#[component]
pub fn LoginRedirect(fragment: ReadOnlySignal<MagicLinkResponse>) -> Element {
    let mut auth = use_context::<AppCtx>().auth_state;
    let mut finished = use_signal(|| false);
    let finish = use_callback(move || finished.set(true));
    use_hook(|| {
        spawn(async move {
            auth.complete_signin(fragment()).await.unwrap();
            finish();
            tracing::debug!("Apparently success?");
            tracing::debug!("auth state: {:?}", auth.email());
            navigator().replace(Route::Learn {});
        });
    });
    rsx! {
        div {
            class: "text-center flex flex-col justify-center items-center gap-4 mb-5 w-full",
            div {
                class: "text-sm text-green-400",
                if finished() {
                    "Login successful!"
                } else {
                    "Logging in..."
                }
            }
        }
    }
}
