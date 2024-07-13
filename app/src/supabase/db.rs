// use std::sync::OnceLock;
// static SB_CLIENT: OnceLock<String> = OnceLock::new();

use gloo_net::http::RequestBuilder;
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::conf::{SUPABASE_ANON_KEY, SUPABASE_API_URL};

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Gloo(#[from] gloo_net::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct SupabaseRequest<T> {
    builder: RequestBuilder,
    select: Option<String>, // TODO: use cow / something more flexible
    _response_type: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> SupabaseRequest<T> {
    pub fn new(table_name: &str) -> Self {
        let builder = RequestBuilder::new(&format!("{SUPABASE_API_URL}/rest/v1/{table_name}"))
            .header("apikey", SUPABASE_ANON_KEY)
            .header("Authorization", &format!("Bearer {SUPABASE_ANON_KEY}"));
        SupabaseRequest {
            builder,
            select: None,
            _response_type: std::marker::PhantomData,
        }
    }

    /// If doing some join or complex query, this can be used to manually cast to the expected type.
    pub fn cast<V>(self) -> SupabaseRequest<V> {
        SupabaseRequest {
            builder: self.builder,
            select: self.select,
            _response_type: std::marker::PhantomData,
        }
    }

    /// Cast to arbitrary json value.
    pub fn json(self) -> SupabaseRequest<serde_json::Value> {
        self.cast()
    }

    // TODO copy docs from postgrest-rs
    pub fn select(mut self, select: &str) -> Self {
        self.select = Some(select.to_string());
        self
    }

    pub async fn execute(self) -> Result<T, Error> {
        let rsp = self
            .builder
            .query([("select", &self.select.expect("missing select"))])
            .send()
            .await?
            .json()
            .await?;
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
