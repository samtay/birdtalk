//! Synchronization logic
//!
//! Immediate state is helded in local storage signals, with async read/writes to remote DB at
//! certain times.
use std::ops::{Deref, DerefMut};

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
use serde::{Deserialize, Serialize};

use crate::{
    stats::Stats,
    supabase::{AuthState, Result, SupabaseRequest, SupabaseResource},
};

#[derive(Clone)]
pub struct Sync<T: 'static> {
    /// Remote backed state
    // remote: Source<T>,
    /// Storage backed state
    local: Signal<T>,
    /// Future to sync remote and local
    fut: UseFuture,
    /// Auth state
    auth: AuthState,
}

impl<T: 'static + std::clone::Clone> Copy for Sync<T> {}

impl Sync<Stats> {
    pub fn init(auth: AuthState) -> Self {
        let local = use_synced_storage::<LocalStorage, _>("stats".to_string(), Stats::default);
        let fut = use_future(move || async move {
            // loop {
            // sync stuff
            // tracing::info!("Auto syncing from long-lived future... TODO");
            // break;
            // }
        });

        Self { local, fut, auth }
        // Super quick "sync" hack: just push whenever user logs in.
        // use_memo(move || {
        //     if auth.is_logged_in() {
        //         spawn(async move { me.sync().await.unwrap() });
        //     }
        // });
    }

    // TODO: function to insert stats with version, returning null|version,
    //       merge and re-push if necessary
    pub async fn sync(&self) -> Result<()> {
        // let mut user_stats = UserStats::new(self.auth, self.local.inner.read().clone());
        // user_stats.push().await?;
        Ok(())
    }
}

impl Deref for Sync<Stats> {
    type Target = Signal<Stats>;

    fn deref(&self) -> &Self::Target {
        &self.local
    }
}

impl DerefMut for Sync<Stats> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.local
    }
}

/// A type to match the public.stats table in the database.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct UserStats {
    user_id: String, // TODO: uuid
    data: Stats,
    updated_at: DateTime<Utc>,
}

impl SupabaseResource for UserStats {
    fn table_name() -> &'static str {
        "stats"
    }
}

// TODO: these requests require overriding the anon jwt with the user's!
impl UserStats {
    pub fn new(auth: AuthState, data: Stats) -> Self {
        Self {
            user_id: auth.user_id().unwrap_or_default(),
            data,
            updated_at: Utc::now(),
        }
    }

    pub async fn fetch(auth: AuthState) -> Result<Self> {
        match auth.user_id() {
            None => Ok(UserStats::default()),
            Some(user_id) => {
                let mut stats = Self::request()
                    .select("*")
                    .eq("user_id", user_id)
                    .execute()
                    .await?;
                Ok(stats.pop().unwrap_or_default())
            }
        }
    }

    pub fn update_stats(&mut self, stats: Stats) -> &mut Self {
        self.data = stats;
        self
    }

    // TODO: throw error on non-200
    pub async fn push(&mut self) -> Result<()> {
        tracing::debug!("Pushing stats for user_id {:?}", self.user_id);
        self.updated_at = Utc::now();
        if !self.user_id.is_empty() {
            SupabaseRequest::<()>::rpc("upsert_stats", self)?
                .execute()
                .await?;
        }
        Ok(())
    }
}
