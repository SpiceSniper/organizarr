// parse_filenames.rs

use std::collections::HashMap;
use std::fs;
use crate::SETTINGS;


/**
 * Contains the file information for each file in the directory.
 * It includes the show name, season, episode, and file extension.
 */
pub struct Fileinfo {
    pub showname: String,
    pub season: String,
    pub episode: String,
    pub extension: String,

}


/**
 * Parses the filenames in a given directory and returns a HashMap
 * containing the file information for each file.
 *
 * @param dir: The directory to parse.
 * @return: A HashMap where the key is the filename and the value is a Fileinfo object.
 */
pub fn parse_files_in_dir(dir: String) -> HashMap<String, Fileinfo> {
    let mut file_map: HashMap<String, Fileinfo> = HashMap::new();

    // Read the directory entries
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Check if the entry is a file
            if !path.is_file() {
                continue;
            }
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                // Check if the file has a valid video or image extension
                if !(SETTINGS.video_extensions.contains(&extension.to_string())
                  || SETTINGS.image_extensions.contains(&extension.to_string())) {
                    continue;
                }
                // Create a Fileinfo object for the file
                let file_info = Fileinfo {
                    // TODO: Add parsing for these features.
                    showname: String::new(), // Populate as needed
                    season: String::new(),   // Populate as needed
                    episode: String::new(),  // Populate as needed
                    extension: extension.to_string(),
                };

                // Insert the Fileinfo into the map
                file_map.insert(path.to_string_lossy().to_string(), file_info);
            }
        }
    }

    file_map
}
