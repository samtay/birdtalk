use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
use serde::{Deserialize, Deserializer, Serialize};
use serde_querystring::UrlEncodedQS;
use thiserror::Error;
use wasm_bindgen::JsValue;

use crate::conf::{self, APP_URL, SUPABASE_ANON_KEY, SUPABASE_API_URL};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    sb_user: SupabaseUser,
    tokens: Tokens,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tokens {
    access_token: String,
    refresh_token: String,
    expires_at: u64,
    expires_in: u64,
    token_type: String,
}

#[derive(Copy, Clone, Debug)]
pub struct AuthState(Signal<AuthStatus>);

// Hopefully this is OK might need a separate layer for use_resource..
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum AuthStatus {
    #[default]
    None,
    Refreshing,
    InvalidToken,
    SignedIn(User),
    SignedOut,
}

// TODO: make cases out of possible responses.
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Gloo(#[from] gloo_net::Error),
}

impl AuthState {
    pub fn init() -> Self {
        // TODO: see _setSession in sb/auth/go-true-client.
        // - decodes jwt to see if expired, if so uses refresh token to get new one.
        // - this function calls _refreshAccessToken
        // TODO: implementation seems fragile, based on line of code. may want to actually key this
        // properly.
        // TODO: there might be a "proper" way to do implicit handling of refreshing session.
        // - on demand is probably easiest?
        // - but maybe it is also reasonable to have a "thread" (or dioxus "use_future") that does this automatically, in the background.
        // TODO: check use_singleton_persistent implementation, if fragile to LOC then make a proper static key
        //  gate it on project ref like sb auth does:
        // const defaultStorageKey = `sb-${new URL(this.authUrl).hostname.split('.')[0]}-auth-token`
        // TODO: when we set "Refreshing" we lose persisted data. Is that ok?
        let inner =
            use_synced_storage::<LocalStorage, _>("sb-auth".to_string(), || AuthStatus::None);
        Self(inner)
    }

    pub fn is_logged_in(&self) -> bool {
        matches!(*self.0.read(), AuthStatus::SignedIn(_))
    }

    pub fn is_anonymous(&self) -> bool {
        self.with_user(|u| u.sb_user.is_anonymous).unwrap_or(false)
    }

    pub fn email(&self) -> Option<String> {
        self.with_user(|u| u.sb_user.email.clone()).flatten()
    }

    pub fn user_id(&self) -> Option<String> {
        self.with_user(|u| u.sb_user.id.clone())
    }

    pub fn user(&self) -> Option<User> {
        self.with_user(Clone::clone)
    }

    pub fn with_user<T>(&self, f: impl FnOnce(&User) -> T) -> Option<T> {
        match *self.0.read() {
            AuthStatus::SignedIn(ref user) => Some(f(user)),
            _ => None,
        }
    }

    pub async fn sign_in_with_magic_link(&self, email: String) -> Result<(), AuthError> {
        gloo_net::http::Request::post(&format!("{SUPABASE_API_URL}/auth/v1/otp"))
            .query([("redirect_to", format!("{APP_URL}/login"))])
            .header("apikey", SUPABASE_ANON_KEY)
            .header("Authorization", &format!("Bearer {SUPABASE_ANON_KEY}"))
            .json(&MagicLinkRequest::new(email))?
            .send()
            .await?;
        Ok(())
    }

    pub async fn complete_signin(&mut self, rsp: MagicLinkResponse) -> Result<(), AuthError> {
        self.0.set(AuthStatus::Refreshing);
        let sb_user = self.get_user(&rsp.access_token).await?;
        let user = User {
            sb_user,
            tokens: Tokens {
                access_token: rsp.access_token,
                refresh_token: rsp.refresh_token,
                expires_at: rsp.expires_at,
                expires_in: rsp.expires_in,
                token_type: rsp.token_type,
            },
        };
        self.0.set(AuthStatus::SignedIn(user));
        Ok(())
    }

    async fn get_user(&self, access_token: &str) -> Result<SupabaseUser, AuthError> {
        let rsp: SupabaseUser =
            gloo_net::http::Request::get(&format!("{SUPABASE_API_URL}/auth/v1/user"))
                .header("apikey", SUPABASE_ANON_KEY)
                .header("Authorization", &format!("Bearer {access_token}"))
                .send()
                .await?
                .json()
                .await?;
        Ok(rsp)
    }

    //TODO: refresh token with /token?grant_type=refresh_token
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct SupabaseUser {
    id: String,
    is_anonymous: bool,
    #[serde(default)]
    email: Option<String>,
    user_metadata: UserMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct UserMetadata {
    #[serde(default)]
    pub full_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MagicLinkRequest {
    /// The email used for signing in (will idenitfy the user)
    email: String,
    /// Any arbitrary metadata to attach to the user
    data: serde_json::Value,
    /// Should we auto-create the user if they don't exist?
    create_user: bool,
    /// Captcha
    gotrue_meta_security: MetaSecurity,
}

impl MagicLinkRequest {
    pub fn new(email: String) -> Self {
        Self {
            email,
            data: serde_json::Value::Null,
            create_user: true,
            gotrue_meta_security: MetaSecurity::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MetaSecurity {
    // #[serde(skip_serializing)]
    captcha_token: Option<String>,
}

/// This login data is returned in a fragment following the redirect URL that the auth server
/// redirects to.
// NOTE: Would be nice to `flatten` a common `Token` struct here, however `deserialize_with` causes
// the underlying sdk > storage > postcard deserialization to fail.. so that's fun.
/// TODO: enum with error http://127.0.0.1:3000/#error=access_denied&error_code=403&error_description=Email+link+is+invalid+or+has+expired
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MagicLinkResponse {
    access_token: String,
    refresh_token: String,
    // TODO: translate to NaiveDateTime like thesurf.in
    #[serde(deserialize_with = "via_string")]
    expires_at: u64,
    #[serde(deserialize_with = "via_string")]
    expires_in: u64,
    token_type: String,
    r#type: String,
}

impl FromHashFragment for MagicLinkResponse {
    fn from_hash_fragment(hash: &str) -> Self {
        tracing::error!("hash {hash}");
        UrlEncodedQS::parse(hash.as_bytes()).deserialize().unwrap()
    }
}

impl Display for MagicLinkResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                ("type", &self.r#type),
                ("access_token", &self.access_token),
                ("refresh_token", &self.refresh_token),
                ("expires_at", &self.expires_at.to_string()),
                ("expires_in", &self.expires_in.to_string()),
                ("token_type", &self.token_type)
            ]
            .map(|(k, v)| format!("{k}={v}"))
            .join("&")
        )
    }
}

fn via_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}
