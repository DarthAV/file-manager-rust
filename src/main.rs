use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <target_directory>");
        std::process::exit(1);
    }

    let target_directory: &str = args[1].as_str();
    let target_path: &Path = Path::new(target_directory);

    if !target_path.exists() {
        eprintln!("The target directory does not exist.");
        std::process::exit(1);
    }
    if !target_path.is_dir() {
        eprintln!("The target directory is not a directory.");
        std::process::exit(1);
    }

    let files = get_files_in_directory(&target_directory);

    println!(
        "Found {} files in directory '{}'",
        files.len(),
        target_directory
    );
    for file_path in files.iter() {
        println!("{}", file_path.display());
    }
}

fn get_files_in_directory(target_directory: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let paths = std::fs::read_dir(target_directory).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let sub_files = get_files_in_directory(path.to_str().unwrap());
            files.extend(sub_files);
        } else {
            files.push(path);
        }
    }
    return files;
}
