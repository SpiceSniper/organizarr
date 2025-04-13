// seaonize.rs

use std::collections::HashMap;
use crate::parse_filenames::Fileinfo;
use crate::Status;

/**
 * This function organizes files into season directories based on their season number.
 * It creates a new directory for each season and moves the files into the corresponding directory.
 *
 * @param dir: The directory to process.
 * @param filenames: A HashMap containing the file paths and their corresponding Fileinfo objects.
 * @return: A Status indicating the result of the operation.
 */
pub fn seasonize(_dir: &String, filenames: &HashMap<String, Fileinfo>) -> Status {

    use std::fs;
    use std::path::Path;
    use crate::SETTINGS;
    let mut existing_season_dirs: HashMap<String, String> = HashMap::new();

    // Run through all detected files in the directory.
    for (file_path, _file_info) in filenames {
        let file_path = Path::new(file_path);
        if let Some(parent_dir) = file_path.parent() {

            // New String to store season directory name.
            let season_dir_name: String;
            let try_season_dir = existing_season_dirs.get(&_file_info.season);
            if try_season_dir.is_none() {

                // No files from this season have been handled yet.
                season_dir_name = adapt_season_name(SETTINGS.season_dir_name.as_str(), _file_info.season.parse::<u32>().unwrap());
                if parent_dir.file_name().and_then(|name| name.to_str()) != Some(season_dir_name.as_str()) {
                    let new_season_dir = parent_dir.join(season_dir_name.clone());

                    // Season dir doesn't exist, create it.
                    if !new_season_dir.exists() {
                        // Might fail due to insuffient permissions.
                        match fs::create_dir_all(&new_season_dir) {
                            Ok(_) => {
                                // Successfully created the directory.
                                println!("Created season directory: {}", new_season_dir.display());
                            }
                            Err(e) => {
                                eprintln!("Failed to create season directory: {}", e);
                                return Status::Error;
                            }
                        }
                    }
                    // Add the new season directory to the map.
                    existing_season_dirs.insert(_file_info.season.clone(), season_dir_name.clone());
                } else {
                    // The file is already detected to be in a season folder,
                    // This can only happen on the first file, so we can return.
                    // this means that all files are already in a season folder.
                    // TODO: Implement Season folder correction -> if files are in a wrongly named folder, or in a folder with the wrong season number.
                    return Status::Ok;
                }
            } else {
                // Season dir already exists, use the existing name.
                season_dir_name = try_season_dir.unwrap().to_string();
            }

            // In any case, we now have a valid season directory name.
            // So construct new path
            let new_file_path = parent_dir.join(season_dir_name).join(file_path.file_name().unwrap());
            match fs::rename(file_path, &new_file_path) {
                Ok(_) => {
                    // Successfully moved the file.
                    println!("Moved file: {} to {}", file_path.display(), new_file_path.display());
                }
                Err(e) => {
                    eprintln!("Failed to move file: {} to {}: {}", file_path.display(), new_file_path.display(), e);
                    return Status::Error;
                }
            }
        }
    }
    Status::FilesChanged
}


/**
 * This function adapts the season name by replacing 'X' characters with the season number.
 * It formats the season number with leading zeros based on the number of 'X' characters.
 *
 * @param season_name: The original season name containing 'X' characters.
 * @param season_number: The season number to replace 'X' with.
 * @return: A new string with 'X' replaced by the formatted season number.
 */
fn adapt_season_name(season_name: &str, season_number: u32) -> String {
    let mut result = String::new();
    let mut x_count = 0;

    for c in season_name.chars() {
        if c == 'X' {
            x_count += 1;
        } else {
            if x_count > 0 {
                // Format the season number with leading zeros based on x_count
                let format_string = format!("{{:0{}d}}", x_count);
                let formatted_number = format!("{} {}", &format_string, season_number);
                result.push_str(&formatted_number);
                x_count = 0;
            }
            result.push(c);
        }
    }

    // Handle any trailing X characters
    if x_count > 0 {
        let format_string = format!("{{:0{}d}}", x_count);
        let formatted_number = format!("{} {}", &format_string, season_number);
        result.push_str(&formatted_number);
    }

    result
}