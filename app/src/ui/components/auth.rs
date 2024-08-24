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
// TODO: FB / Google / Apple
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
                class: "w-64 px-3 py-2 rounded-lg border-none",
                "type": "email",
                name: "email",
                placeholder: "Enter your email",
                oninput: move |event| email.set(event.value())
            }
            button {
                class: "px-4 py-2 focus:outline-none focus-visible:ring focus-visible:ring-green-400 font-semibold text-base bg-green-dark text-white rounded-full shadow",
                disabled: magic_link_sent(),
                "Login / Signup"
            }
            if magic_link_sent() {
                div {
                    class: "text-sm",
                    "We sent a magic link to your email! Check your inbox and click the link to login."
                }
            }
        }
    }
}

/// A component for receiving the magic link response and redirecting back to the main app.
// TODO: sync stats (pull, merge, push) after login
//       (possibly a view of local and remote stats, option to merge?)
// TODO: don't redirect, just say "login successful, you can close this page now! Or go back [[home]]"
// TODO: when ready, add back as a route:
// #[route("/login/#:fragment")]
// LoginRedirect {
//     fragment: MagicLinkResponse
// },
#[component]
pub fn LoginRedirect(fragment: ReadOnlySignal<MagicLinkResponse>) -> Element {
    let mut auth = use_context::<AppCtx>().auth_state;
    let mut finished = use_signal(|| false);
    let finish = use_callback(move |_| finished.set(true));
    use_hook(|| {
        spawn(async move {
            auth.complete_signin(fragment()).await.unwrap();
            finish(());
            tracing::debug!("Apparently success?");
            tracing::debug!("auth state: {:?}", auth.email());
            navigator().replace(Route::Index {});
        });
    });
    rsx! {
        div {
            class: "text-center flex flex-col justify-center items-center gap-4 mb-5 w-full",
            div {
                class: "text-sm text-green",
                if finished() {
                    "Login successful!"
                } else {
                    "Logging in..."
                }
            }
        }
    }
}

// E.g. #[layout(LoginGate)]
#[component]
fn LoginGate() -> Element {
    let ctx = use_context::<AppCtx>();
    let on_open_route = matches!(use_route(), Route::Index {});
    let logged_in = use_memo(move || ctx.auth_state.is_logged_in());
    let login_needed = !on_open_route && !logged_in();
    // TODO: Perhaps arbitrarily delay to second generation() for SSG?
    //       Pending / refreshing -> Learn view shows placeholders
    //       Signed out           -> Fetch free packs with anon key
    //       Signed in            -> Fetch packs relevant to user
    rsx! {
        if login_needed {
            div {
                class: "flex flex-col items-center justify-center h-full",
                Login {}
            }
        } else {
            Outlet::<Route> { }
        }
    }
}
