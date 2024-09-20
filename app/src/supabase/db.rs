use std::fmt::Display;

use gloo_net::http::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::{
    conf::{SUPABASE_ANON_KEY, SUPABASE_API_URL},
    utils,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Gloo(#[from] gloo_net::Error),
    #[error("Uh oh! We couldn't find today's pack!")]
    NoDailyPack,
    #[allow(clippy::enum_variant_names)]
    #[error("{0}")]
    ErrorMessage(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::ErrorMessage(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::ErrorMessage(s.to_string())
    }
}

// Dioxus is very awkward without clonable errors, so unfortuantely we'll lose a bunch of info and turn things into just error messages.
impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Self::Gloo(e) => Self::ErrorMessage(e.to_string()),
            Self::NoDailyPack => Self::NoDailyPack,
            Self::ErrorMessage(msg) => Self::ErrorMessage(msg.clone()),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct SupabaseRequest<T = serde_json::Value> {
    builder: RequestBuilder,
    /// Gloonet annoyingly turns a RequestBuilder into a Request when setting the body, so we
    /// postpone this until the end (to support adding filters before executing).
    body: Option<String>,
    _response_type: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> SupabaseRequest<T> {
    /// Create a REST request _from_ the specified Supabase table/view.
    pub fn from(table_name: &str) -> Self {
        let builder = gloo_net::http::RequestBuilder::new(&format!(
            "{SUPABASE_API_URL}/rest/v1/{table_name}"
        ))
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", &format!("Bearer {SUPABASE_ANON_KEY}"));
        SupabaseRequest {
            builder,
            body: None,
            _response_type: std::marker::PhantomData,
        }
    }

    /// Call a Postgres function.
    pub fn rpc<F, U>(function: F, params: &U) -> Result<Self>
    where
        F: Display,
        U: Serialize,
    {
        let builder = gloo_net::http::RequestBuilder::new(&format!(
            "{SUPABASE_API_URL}/rest/v1/rpc/{function}"
        ))
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", &format!("Bearer {SUPABASE_ANON_KEY}"));
        let json = serde_json::to_string(params).map_err(gloo_net::Error::from)?;
        Ok(SupabaseRequest {
            builder,
            body: Some(json),
            _response_type: std::marker::PhantomData,
        })
    }

    /// If doing some join or complex query, this can be used to manually cast to the expected type.
    pub fn cast<V>(self) -> SupabaseRequest<V> {
        SupabaseRequest {
            builder: self.builder,
            body: self.body,
            _response_type: std::marker::PhantomData,
        }
    }

    /// Cast to arbitrary json value.
    pub fn json(self) -> SupabaseRequest<serde_json::Value> {
        self.cast()
    }

    /// Select columns
    pub fn select<C>(mut self, columns: C) -> Self
    where
        C: AsRef<str>,
    {
        self.builder = self.builder.query([("select", columns)]);
        self
    }

    /// Add equality filter
    pub fn eq<C, D>(mut self, column: C, filter: D) -> Self
    where
        C: AsRef<str>,
        D: Display,
    {
        self.builder = self
            .builder
            .query([(column.as_ref(), &format!("eq.{}", filter))]);
        self
    }

    /// Add IN array filter
    pub fn in_<C, I, D>(mut self, column: C, values: I) -> Self
    where
        C: AsRef<str>,
        I: IntoIterator<Item = D>,
        D: Display,
    {
        self.builder = self.builder.query([(
            column.as_ref(),
            &format!("in.({})", utils::join(values, ",")),
        )]);
        self
    }

    /// Execute request
    pub async fn execute(self) -> Result<T, Error> {
        let req = if let Some(body) = self.body {
            self.builder.body(body)
        } else {
            self.builder.build()
        }?;
        let rsp = req.send().await?.json().await?;
        Ok(rsp)
    }
}

/// Indicates a type corresponds to a RESTful resource, i.e. a table in Supabase.
pub trait SupabaseResource: Sized + DeserializeOwned {
    fn table_name() -> &'static str;

    // TODO: When auth is implemented, there will probably be some state to reference the correct
    // auth token.
    fn request() -> SupabaseRequest<Vec<Self>> {
        SupabaseRequest::from(Self::table_name())
    }
}
