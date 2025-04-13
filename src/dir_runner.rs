// dir_runner.rs

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use futures::future::join_all;
use tokio::task;
use crate::parse_filenames::Fileinfo;
use crate::Status;


/**
 * This function processes directories and executes tasks on them.
 * It takes a vector of directory paths and a vector of tasks to execute.
 * The tasks are executed synchronously for each directory, and then the function
 * collects all subdirectories and processes them asynchronously.
 * This utiliyes asznchronous tree recursion, for maximum utiliyation of multicore processors.
 *
 * @param dirs: A vector of directory paths to process.
 * @param tasks: A vector of functions to execute on each directory.
 * @return: A future that resolves to true when all tasks are completed.
 */
pub fn process_directories(
    dirs: Vec<String>,
    tasks: Vec<fn(&String, &HashMap<String, Fileinfo>) -> Status>,
) -> Pin<Box<dyn Future<Output = bool> + Send>> {
    Box::pin(async move {

        // Vector to store handles of asynchronous tasks.
        let mut handles = vec![];

        // Go through all dirs in the vector.
        // This is only used on the first level of the recursion.
        for dir in &dirs {
            // Execute all tasks for the current directory synchronously,
            // because task order might be relevant.
            run_tasks(tasks.clone(), dir);

            // Collect subdirectories
            let subdirs = get_subdirs(dir);

            // Asynchronously process each subdirectory
            for dir in subdirs {
                let handle = task::spawn(process_directories(vec![dir], tasks.clone()));
                handles.push(handle);
            }
        }

        // Wait for all asynchronous tasks to complete
        let _ = join_all(handles).await;
        true
    })
}


/**
 * This function runs a vector of tasks synchronously.
 * It takes a vector of functions to execute and returns a boolean indicating success or failure.
 *
 * @param tasks: A vector of functions to execute.
 * @return: A boolean indicating success or failure.
 */
fn run_tasks(tasks: Vec<fn(&String, &HashMap<String, Fileinfo>) -> Status>, dir: &String) {
    let mut files = crate::parse_filenames::parse_files_in_dir(dir.to_string());
    for task in tasks {
        if let Status::FilesChanged = task(dir, &files) {
            // Signals that some files changed, reread the directory.
            files = crate::parse_filenames::parse_files_in_dir(dir.to_string());
        }
    }
}

/**
 * This function retrieves all subdirectories of a given directory.
 * It takes a directory path as input and returns a vector of subdirectory paths.
 *
 * @param dir: A string representing the directory path.
 * @return: A vector of strings representing the subdirectory paths.
 */
fn get_subdirs(dir: &String) -> Vec<String> {
    let entries = std::fs::read_dir(dir).into_iter().flatten();
    entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let name = entry.file_name().into_string().ok()?;
            // Only get not-ignored directories.
            if path.is_dir() && !is_ignored(&path, &name) {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect()
}


/**
 * This function checks if a given path or name is ignored based on the settings.
 * It takes a path and a name as input and returns a boolean indicating if it is ignored.
 *
 * @param path: A reference to a Path representing the directory path.
 * @param name: A string representing the name of the directory.
 * @return: A boolean indicating if the path or name is ignored.
 */
fn is_ignored(path: &std::path::Path, name: &str) -> bool {
    let ignored_dirs = &crate::SETTINGS.ignored_directories;

    for ignored in ignored_dirs {

        // Check for ignored absolute paths.
        if ignored.starts_with('/') {
            // Absolute path match
            if path == std::path::Path::new(ignored) {
                return true;
            }

        // Check for ignored relative paths.
        } else if ignored.starts_with('.') {
            // Relative path match
            let relative_path = ignored.trim_start_matches('.');
            for target_dir in &crate::SETTINGS.target_dirs {
                let full_path = std::path::Path::new(target_dir).join(relative_path);
                if path == full_path {
                    return true;
                }
            }
        } else {

            // Check for ignored directory names.
            if name == ignored {
                return true;
            }
        }
    }

    false
}