use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Bird {
    pub common_name: String,
    pub scientific_name: String,
    // TODO: vec of sounds, each with enum of Call/Song etc. Probably also want link to src, name
    // for credit, CC type, etc.
    pub sound_file: PathBuf,
    // TODO: vec of imgs?
    pub img_file: PathBuf,
}

pub fn demo_data() -> Vec<Bird> {
    serde_json::from_str(include_str!("../data/demo.json")).unwrap()
}
