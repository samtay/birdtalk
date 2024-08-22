use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::supabase::{self, Result, SupabaseResource};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bird<S> {
    pub id: u64,
    pub common_name: String,
    pub scientific_name: String,
    pub image: String,
    pub sounds: S,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    pub path: String,
    pub default_: bool,
}

pub type BirdSummary = Bird<()>;
pub type BirdDetailed = Bird<Vec<Sound>>;

impl<S> PartialEq for Bird<S> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<S> Bird<S> {
    /// Get image URL hosted by Supabase storage, e.g. http://127.0.0.1:54321/storage/v1/object/public/bird_images/cardinalis-cardinalis/unlicensed-optimized.jpg
    pub fn image_url(&self) -> String {
        supabase::storage_object_url(&self.image)
    }
}

impl BirdDetailed {
    pub fn default_sound_url(&self) -> String {
        supabase::storage_object_url(&self.sounds[0].path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdPack<B> {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub birds: Vec<B>,
}

impl<B> PartialEq for BirdPack<B> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub type BirdPackSummary = BirdPack<BirdSummary>;
pub type BirdPackDetailed = BirdPack<BirdDetailed>;

impl BirdPack<BirdSummary> {
    /// Query db for free packs
    pub async fn fetch_free_packs() -> Vec<Self> {
        todo!()
    }
}

impl SupabaseResource for BirdPackDetailed {
    fn table_name() -> &'static str {
        "bird_packs_detailed"
    }
}

impl BirdPackDetailed {
    /// Query db for bird pack by id
    pub async fn fetch_by_id(id: u64) -> Result<Self> {
        Self::request()
            .cast::<BirdPackDetailed>()
            .select("*")
            .eq("id", id.to_string())
            .execute()
            .await
    }

    /// Query db for free packs
    // TODO: filter free = true
    pub async fn fetch_free_packs() -> Result<Vec<Self>> {
        Self::request().select("*").order("id.asc").execute().await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BirdPackDaily {
    pub pack: BirdPackDetailed,
    pub day: NaiveDate,
}

impl SupabaseResource for BirdPackDaily {
    fn table_name() -> &'static str {
        "daily_packs"
    }
}

impl BirdPackDaily {
    /// Query db for pack of the day (respects local time)
    pub async fn fetch_today() -> Result<Self> {
        #[derive(Debug, Deserialize)]
        pub struct WrappedPack {
            bird_packs_detailed: BirdPackDetailed,
        }

        let day = chrono::offset::Local::now().date_naive();
        let pack = Self::request()
            .cast::<Vec<WrappedPack>>()
            .select("bird_packs_detailed(*)")
            .eq("day", day.format("%Y-%m-%d").to_string())
            .execute()
            .await?
            .pop()
            .ok_or_else(|| supabase::Error::NoDailyPack)?
            .bird_packs_detailed;
        Ok(Self { pack, day })
    }
}
