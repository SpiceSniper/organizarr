// settings.rs

use serde::Deserialize;
use lazy_static::lazy_static;
use std::fs;


/**
 * All settings are stored here during runtime.
 * The settings are read from the settings.yaml file.
 * For argument descriptions, see the settings.yaml file.
 */
#[derive(Deserialize)]
pub struct Settings {
    pub default_args: Vec<String>,
    pub target_dirs: Vec<String>,
    pub video_extensions: Vec<String>,
    pub image_extensions: Vec<String>,
    pub ignored_directories: Vec<String>,
    pub season_dir_name: String
}

lazy_static! {
    /**
     * Global instane of SETTINGS, holding all values parsed from settings.yaml.
     */
    pub static ref SETTINGS: Settings = {
        let config_str = fs::read_to_string("src/settings.yaml")
            .expect("Failed to read settings.yaml");
        serde_yaml::from_str(&config_str)
            .expect("Failed to parse settings.yaml")
    };
}