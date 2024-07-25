//! Synchronization logic: an "offline-first" esque approach to handling state. Immediate state is
///helded in local storage signals, with async read/writes to remote DB.
// TODO: Centralized user "model" that holds synced storage backing and cached remote data;
// Writes can go through the model, first update the local storage, invalidate some caches, and kick off future to update remote.
// If remote has later timestamp than most recently received cache, will need to pull in updates. (async non-blocking)
// This will need to be done at time of local storage init too.
use std::ops::{Deref, DerefMut};

// Or just use React Native and get those sweet supabase integrations for local-first embedded
// sqlite approaches.
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use crate::{
    stats::{Stats, UserStats},
    supabase::{AuthState, Result},
};

#[derive(Clone, Debug)]
pub struct Source<T: 'static> {
    /// State (e.g. storage backed / db backed)
    inner: Signal<T>,
    updated_at: Signal<DateTime<Utc>>,
}

impl<T: 'static + std::clone::Clone> Copy for Source<T> {}

impl Source<Stats> {
    pub fn local() -> Self {
        let inner = use_synced_storage::<LocalStorage, _>("stats".to_string(), Stats::default);
        let updated_at = use_signal(Utc::now);
        Self { inner, updated_at }
    }
}

#[derive(Clone)]
pub struct Sync<T: 'static> {
    /// Remote backed state
    remote: Source<T>,
    /// Storage backed state
    local: Source<T>,
    /// Future to sync remote and local
    fut: UseFuture,
    ///
    auth: AuthState,
}

impl<T: 'static + std::clone::Clone> Copy for Sync<T> {}

impl Sync<Stats> {
    // TODO: pull auth from ctx?
    pub fn init(auth: AuthState) -> Self {
        // let _key = auth.email().unwrap_or_else(|| "anon".to_string());
        // let remote_stats = use_synced_storage::<RemoteStorage, _>("{user.email}".to_string(), Stats::default);
        let local = Source::<Stats>::local();
        let remote = Source::<Stats>::local(); // TODO: remote
        let fut = use_future(move || async move {
            loop {
                // sync stuff
                tracing::info!("Auto syncing from long-lived future... TODO");
                break;
            }
        });

        let me = Self {
            remote,
            local,
            fut,
            auth,
        };
        // Super quick "sync" hack: just push whenever user logs in.
        use_memo(move || {
            if auth.is_logged_in() {
                spawn(async move { me.sync().await.unwrap() });
            }
        });
        me
    }

    pub async fn sync(&self) -> Result<()> {
        let mut user_stats = UserStats::new(self.auth, self.local.inner.read().clone());
        user_stats.push().await?;
        Ok(())
    }
}

impl Deref for Sync<Stats> {
    type Target = Signal<Stats>;

    fn deref(&self) -> &Self::Target {
        &self.local.inner
    }
}

impl DerefMut for Sync<Stats> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.local.inner
    }
}
