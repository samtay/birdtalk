// use std::sync::OnceLock;
// static SB_CLIENT: OnceLock<String> = OnceLock::new();

use once_cell::sync::Lazy;
use postgrest::{Builder, Postgrest};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::conf::{SUPABASE_ANON_KEY, SUPABASE_API_URL};

static POSTGREST_CLIENT: Lazy<Postgrest> = Lazy::new(|| {
    Postgrest::new(format!("{SUPABASE_API_URL}/rest/v1"))
        .insert_header("apikey", SUPABASE_ANON_KEY)
        .insert_header("Authorization", format!("Bearer {SUPABASE_ANON_KEY}"))
});

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Gloo(#[from] gloo_net::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct SupabaseRequest<T> {
    builder: Builder,
    _response_type: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> SupabaseRequest<T> {
    pub fn new(table_name: &str) -> Self {
        let builder = POSTGREST_CLIENT.from(table_name);
        SupabaseRequest {
            builder,
            _response_type: std::marker::PhantomData,
        }
    }

    /// If doing some join or complex query, this can be used to manually cast to the expected type.
    pub fn cast<V>(self) -> SupabaseRequest<V> {
        SupabaseRequest {
            builder: self.builder,
            _response_type: std::marker::PhantomData,
        }
    }

    /// Cast to arbitrary json value.
    pub fn json(self) -> SupabaseRequest<serde_json::Value> {
        self.cast()
    }

    /// See [`Builder::select`]
    pub fn select<C>(mut self, columns: C) -> Self
    where
        C: Into<String>,
    {
        self.builder = self.builder.select(columns);
        self
    }

    /// See [`Builder::order`]
    pub fn order<C>(mut self, columns: C) -> Self
    where
        C: Into<String>,
    {
        self.builder = self.builder.order(columns);
        self
    }

    pub async fn execute(self) -> Result<T, Error> {
        let rsp = self.builder.execute().await?.json().await?;
        Ok(rsp)
    }
}

pub trait SupabaseResource: Sized + DeserializeOwned {
    fn table_name() -> &'static str;

    // When auth is implemented, there will probably be some state to reference the correct
    // auth token.
    fn request() -> SupabaseRequest<Vec<Self>> {
        SupabaseRequest::new(Self::table_name())
    }
}
