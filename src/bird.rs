use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bird {
    pub common_name: String,
    pub scientific_name: String,
    // TODO: vec of sounds, each with enum of Call/Song etc. Probably also want link to src, name
    // for credit, CC type, etc.
    pub sound_file: String,
    // TODO: vec of imgs?
    pub img_file: String,
}

impl Bird {
    pub fn id(&self) -> &str {
        &self.scientific_name
    }
}

impl PartialEq for Bird {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BirdPack {
    pub id: String,
    pub name: String,
    pub description: String,
    pub birds: Vec<Bird>,
    pub already_learned: bool,
}

impl BirdPack {
    pub fn demo() -> Self {
        Self {
            id: "demo".to_string(),
            name: "Demo Pack".to_string(),
            description: "A selection of common birds in North America".to_string(),
            birds: demo_data(),
            already_learned: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

pub fn demo_data() -> Vec<Bird> {
    serde_json::from_str(include_str!("../data/demo.json")).unwrap()
}
