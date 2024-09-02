use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::supabase::{self, Error, Result, SupabaseResource};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bird {
    pub id: u64,
    pub common_name: String,
    pub scientific_name: String,
    pub image: String,
    pub sounds: Vec<Sound>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    pub path: String,
    pub default_: bool,
}

impl PartialEq for Bird {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Bird {
    /// Get image URL hosted by Supabase storage, e.g. http://127.0.0.1:54321/storage/v1/object/public/bird_images/cardinalis-cardinalis/unlicensed-optimized.jpg
    pub fn image_url(&self) -> String {
        supabase::storage_object_url(&self.image)
    }

    pub fn default_sound_url(&self) -> String {
        supabase::storage_object_url(&self.sounds[0].path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdPack {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub birds: Vec<Bird>,
    pub day: Option<NaiveDate>,
}

impl PartialEq for BirdPack {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl SupabaseResource for BirdPack {
    fn table_name() -> &'static str {
        "bird_packs_detailed"
    }
}

impl BirdPack {
    /// Query db for bird pack by id
    pub async fn fetch_by_id(id: u64) -> Result<Self> {
        Self::request()
            .select("*")
            .eq("id", id.to_string())
            .execute()
            .await?
            .pop()
            .ok_or_else(|| Error::from(format!("Drats! No pack found with id {id}")))
    }

    /// Query db for pack of the day (respects local time)
    pub async fn fetch_today() -> Result<Self> {
        let day = chrono::offset::Local::now().date_naive();
        Self::request()
            .select("*")
            .eq("day", day.format("%Y-%m-%d").to_string())
            .execute()
            .await?
            .pop()
            .ok_or_else(|| supabase::Error::NoDailyPack)
    }
}
