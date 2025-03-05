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

    let files = get_dir_tree(&target_path);

    println!(
        "Found {} files in directory '{}'",
        files.len(),
        target_directory
    );
    for file_path in files.iter() {
        let mut path_string: String = file_path.display().to_string();
        path_string = path_string.replacen(target_directory, "", 1);

        if file_path.is_dir() {
            println!("{}", path_string);
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
