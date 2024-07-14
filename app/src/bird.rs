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
    pub async fn fetch_by_id(_id: u64) -> Result<Self> {
        todo!()
    }

    /// Query db for free packs
    // TODO: filter free = true
    pub async fn fetch_free_packs() -> Result<Vec<Self>> {
        Self::request().select("*").order("id.asc").execute().await
    }
}
