use std::{fs, io, path::PathBuf};

const SELECTED_DIR_NAMES: [&str; 4] = ["node_modules", "dist", ".next", ".turbo"];

/// Recursively finds directories matching a specific name within a given path.
fn find_matching_dirs_recursive(
    path: &PathBuf,
    selected_dir_name: &str,
    skip_dirs: &[PathBuf],
) -> io::Result<Vec<PathBuf>> {
    let mut found_paths = Vec::new();

    let is_legit_dir = path.is_dir();

    if !is_legit_dir {
        // If it's not a directory or doesn't exist, we can't search it.
        return Ok(found_paths);
    }

    let entries = fs::read_dir(path)?;

    for entry_result in entries {
        let entry = entry_result?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // Skip if this directory is already marked for deletion
            if skip_dirs.iter().any(|skip| skip == &entry_path) {
                continue;
            }
            // Check if the directory name is selected_dir_name
            if let Some(dir_name) = entry_path.file_name() {
                if dir_name == selected_dir_name {
                    found_paths.push(entry_path.clone()); // Found it!
                    continue;
                }
            }
            // Recursively search inside this subdirectory, regardless of its name
            found_paths.extend(find_matching_dirs_recursive(
                &entry_path,
                selected_dir_name,
                skip_dirs,
            )?);
        }
    }

    Ok(found_paths)
}

/// Pure logic - finds all selected directory types without I/O side effects
fn find_all_selected_dirs(given_path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut selected_folders = Vec::new();
    let mut skip_dirs = Vec::new();

    for &dir_name in &SELECTED_DIR_NAMES {
        let found = find_matching_dirs_recursive(given_path, dir_name, &skip_dirs)?;
        selected_folders.extend(found.clone());
        skip_dirs.extend(found);
    }

    Ok(selected_folders)
}

/// Handles the search process - manages UI/printing
pub fn handle_searching_dirs(given_path: PathBuf) -> io::Result<Vec<PathBuf>> {
    println!("\nSearching for directories in: {}", given_path.display());

    match find_all_selected_dirs(&given_path) {
        Ok(folders) => Ok(folders),
        Err(e) => {
            eprintln!("Error searching directories: {}", e);
            Err(e)
        }
    }
}
