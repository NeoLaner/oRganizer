use std::{
    fs,
    io::{self, stdin},
    path::PathBuf,
};

// Function to delete a list of directories
fn delete_dirs(paths: Vec<PathBuf>) -> io::Result<()> {
    if paths.is_empty() {
        println!("No directories folders found to delete.");
        return Ok(());
    }

    println!("Found directories directories to delete:");
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

pub fn handle_delete_dirs(paths: Vec<PathBuf>) -> io::Result<()> {
    match delete_dirs(paths) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("Error deleting directories: {}", e);
            Err(e)
        }
    }
}
