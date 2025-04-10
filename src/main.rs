use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run <operation>");
        std::process::exit(1);
    }

    let operation: &str = args[1].as_str();
    let target_directory: &str = args[2].as_str();

    if !Path::new(target_directory).exists() {
        eprintln!("Target directory does not exist.");
        std::process::exit(1);
    }

    if operation == "tree" {
        handle_tree(target_directory);
    } else if operation == "flatten" {
        if args.len() < 4 {
            eprintln!("Usage: cargo run flatten <target_directory> <destination_directory>");
            std::process::exit(1);
        }

        let destination_directory = args[3].as_str();

        if !Path::new(destination_directory).exists() {
            std::fs::create_dir(destination_directory).unwrap();
        }

        handle_flatten(target_directory, destination_directory);
    } else if operation == "duplicates" {
        handle_duplicates(target_directory);
    } else {
        eprintln!("Unknown operation: {}", operation);
        std::process::exit(1);
    }
}

// Prints the directory tree of the target directory.
fn handle_tree(target_directory: &str) {
    let target_directory = Path::new(target_directory);
    let files = get_dir_tree(target_directory);
    let file_len = files.len();
    for file in files {
        let relative_path = file.strip_prefix(target_directory).unwrap();
        println!("{}", relative_path.display());
    }
    println!(
        "Found {} files in directory '{}'",
        file_len,
        target_directory.display()
    );
}

// Flatten the directory structure.
fn handle_flatten(target_directory: &str, destination_directory: &str) {
    // copy all files from the target directory to the destination directory.
    let files = get_dir_tree(Path::new(target_directory));
    for file in files {
        if file.is_dir() {
            continue;
        }

        let file_name = file.file_name().unwrap();
        let destination = Path::new(destination_directory).join(file_name);

        if destination.exists() {
            println!(
                "File already exists in destination: {}",
                destination.display()
            );
            continue;
        }

        let result = std::fs::copy(&file, &destination);
        if let Err(error) = result {
            eprintln!("Error copying file: {:?}", error);
        }
    }
}

// Find duplicate files in the directory.
fn handle_duplicates(target_directory: &str) {
    let files = get_dir_tree(Path::new(target_directory));
    let mut file_map: std::collections::HashMap<u64, Vec<PathBuf>> =
        std::collections::HashMap::new();

    // Find duplicate files by size
    for file in files {
        if file.is_dir() {
            continue;
        

        }
    }
}

fn get_dir_tree(target_directory: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let paths = std::fs::read_dir(target_directory).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let sub_files = get_dir_tree(&path);
            files.extend(sub_files);
        }
        files.push(path);
    }
    return files;
}
