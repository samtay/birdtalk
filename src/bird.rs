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

pub fn test_bird_data() -> Vec<Bird> {
    vec![
        Bird {
            common_name: "Cedar Waxwing".to_string(),
            scientific_name: "Bombycilla cedrorum".to_string(),
            sound_file: PathBuf::from("sounds/eurasion_wren.mp3"),
            // sound_file: PathBuf::from("cedar_waxwing.mp3"),
            img_file: PathBuf::from("imgs/cedar_waxwing.jpg"),
        },
        Bird {
            common_name: "Northern Cardinal".to_string(),
            scientific_name: "Cardinalis cardinalis".to_string(),
            // sound_file: PathBuf::from("northern_cardinal.mp3"),
            sound_file: PathBuf::from("sounds/eurasion_wren.mp3"),
            img_file: PathBuf::from("imgs/northern_cardinal.jpg"),
        },
        Bird {
            common_name: "Green Heron".to_string(),
            scientific_name: "Butorides virescens".to_string(),
            sound_file: PathBuf::from("sounds/eurasion_wren.mp3"),
            img_file: PathBuf::from("imgs/green_heron.jpg"),
        },
        Bird {
            common_name: "Mourning Dove".to_string(),
            scientific_name: "Zenaida macroura".to_string(),
            // sound_file: PathBuf::from("mourning_dove.mp3"),
            sound_file: PathBuf::from("sounds/eurasion_wren.mp3"),
            img_file: PathBuf::from("imgs/mourning_dove.jpg"),
        },
    ]
}
