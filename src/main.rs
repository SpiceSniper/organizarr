mod seasonize;
mod settings;
mod dir_runner;
mod parse_filenames;

use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::parse_filenames::Fileinfo;
use crate::seasonize::seasonize;
use crate::settings::SETTINGS;


/** * Status enum to represent the return status of a function.
 * It can be either Ok or Error.
 */
pub enum Status {
    Ok,
    Error,
    FilesChanged,
}

lazy_static!(
    /**
     * Global instance of ARGUMENTS, holding all functions that can be called via command line arguments.
     * The key is the argument character, and the value is the function to be called.
     */
    #[derive(Debug)]
    static ref ARGUMENTS: HashMap<char, fn(&String, &HashMap<String, Fileinfo>) -> Status> = {
        let mut map = HashMap::new();
        map.insert('s', seasonize as fn(&String, &HashMap<String, Fileinfo>) -> Status);
        map
    };
);


/**
 * Main function of the program.
 * It handles command line arguments and executes the corresponding functions.
 * If no arguments are provided, it uses the default arguments from settings.yaml.
 */
#[tokio::main]
async fn main() {

    // Get arguments, or use standard args.
    let args= handle_arguments();

    // For each of the arguments, check if a function exists in the Arguments map.
    // execute the function, if it exists.
    let tasks: Vec<fn(&String, &HashMap<String, Fileinfo>) -> Status> = args
        .iter()
        .filter_map(|arg| ARGUMENTS.get(arg))
        .cloned()
        .collect();

    // Execute all tasks.
    dir_runner::process_directories(SETTINGS.target_dirs.clone(), tasks).await;


    // Finished running all tasks.
    // Print some information about the tasks that were run.
    // If no tasks were run, provide the user with information on how to supply arguments.
    if args.iter().count() == 1 {
        println!("No valid arguments provided. Exiting.");
        println!("If you run this program without arguments, standard arguments must be provided via settings.yaml.");
        println!("If you did provide arguments in via commandline or settings.yaml,\nplease ensure that only the following arguments are used:");
        println!("{:?}", ARGUMENTS);
    } else {
        println!("Finished! Ran {} tasks.", args.len());
    }
}

/**
 * Handles the command line arguments.
 * If no arguments are provided, it will use the default arguments from settings.yaml.
 * If arguments are provided, it will filter them to only include valid ones.
 *
 * @return A vector of characters representing the valid arguments.
 */
fn handle_arguments() -> Vec<char> {
    use std::env::args;

    // Check if no arguments are provided or if the first argument is not a flag.
    if args().count() == 0 || args().count() == 1 && !args().nth(0).unwrap().starts_with('-') {
        // If no arguments are provided, use the default arguments from settings.yaml.
        println!("No arguments provided. Running with standard arguments!");
        SETTINGS.default_args.clone().iter().filter(|arg| arg.starts_with('-') && arg.len() == 2)
            .map(|arg| arg.chars().nth(1).unwrap())
            .collect()
    } else {
        // Filter the arguments to only include valid ones.
        args().filter(|arg| arg.starts_with('-') && arg.len() == 2)
            .map(|arg| arg.chars().nth(1).unwrap())
            .collect()
    }
}