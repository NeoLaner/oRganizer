use std::{
    fs,
    io::{self, stdin},
    path::PathBuf,
};

/// Recursively finds all "node_modules" directories within a given path.
fn find_node_modules_directories(path: &PathBuf) -> Vec<PathBuf> {
    let mut found_paths = Vec::new();

    // Check if the current path is a directory and exists
    if !path.is_dir() {
        // If it's not a directory or doesn't exist, we can't search it.
        // You might want to log this or return an error if it's the initial call.
        return found_paths;
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let entry_path = entry.path();
                        if entry_path.is_dir() {
                            // Check if the directory name is "node_modules"
                            if let Some(dir_name) = entry_path.file_name() {
                                if dir_name == "node_modules" {
                                    found_paths.push(entry_path.clone()); // Found it!
                                    continue;
                                }
                            }
                            // Recursively search inside this subdirectory, regardless of its name
                            found_paths.extend(find_node_modules_directories(&entry_path));
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading directory entry in {}: {}", path.display(), e);
                        // Continue to next entry or break, depending on desired error handling
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory {}: {}", path.display(), e);
            // This error means we couldn't even start reading the directory.
        }
    }

    found_paths
}

// Function to delete a list of directories
fn delete_directories(paths: Vec<PathBuf>) -> io::Result<()> {
    if paths.is_empty() {
        println!("No 'node_modules' folders found to delete.");
        return Ok(());
    }

    println!("Found 'node_modules' directories to delete:");
    for p in &paths {
        println!("- {}", p.display());
    }

    println!("\nAre you sure you want to delete these directories? (y/N)");
    let mut confirmation = String::new();
    stdin().read_line(&mut confirmation)?;

    if confirmation.trim().to_lowercase() == "y" {
        for path_to_delete in paths {
            println!("Deleting: {}", path_to_delete.display());
            match fs::remove_dir_all(&path_to_delete) {
                Ok(_) => println!("Successfully deleted: {}", path_to_delete.display()),
                Err(e) => eprintln!("Error deleting {}: {}", path_to_delete.display(), e),
            }
        }
        println!("\nDeletion process complete.");
    } else {
        println!("Deletion cancelled.");
    }

    Ok(())
}


fn main() -> io::Result<()> {
    println!("oRganizer");
    let mut path_input: String = String::new();
    println!("Enter the base directory path to search for 'node_modules' (e.g., . for current directory):");
    stdin().read_line(&mut path_input)?;

    let base_path = PathBuf::from(path_input.trim());

    if !base_path.exists() {
        eprintln!("Error: The specified path '{}' does not exist.", base_path.display());
        return Ok(());
    }

    if !base_path.is_dir() {
        eprintln!("Error: The specified path '{}' is not a directory. Please enter a directory path.", base_path.display());
        return Ok(());
    }

    println!("\nSearching for 'node_modules' folders in: {}", base_path.display());
    let node_modules_folders = find_node_modules_directories(&base_path);

    delete_directories(node_modules_folders)?;

    Ok(())
}
