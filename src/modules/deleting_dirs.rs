use std::{
    fs,
    io::{self, stdin},
    path::PathBuf,
};

// Pure logic - deletes directories based on user confirmation
fn delete_dirs(paths: Vec<PathBuf>) -> io::Result<()> {
    for path_to_delete in paths {
        fs::remove_dir_all(&path_to_delete)?;
    }

    Ok(())
}

fn get_user_confirmation() -> io::Result<bool> {
    let mut confirmation = String::new();
    stdin().read_line(&mut confirmation)?;
    Ok(confirmation.trim().to_lowercase() == "y")
}

pub fn handle_delete_dirs(paths: Vec<PathBuf>) -> io::Result<()> {
    if paths.is_empty() {
        println!("No directories found to delete.");
        return Ok(());
    }

    println!("Directories to delete:");
    for p in &paths {
        println!("- {}", p.display());
    }

    println!("\nAre you sure you want to delete these directories? (y/N)");
    let confirmed = get_user_confirmation()?;

    if confirmed {
        for path in &paths {
            println!("Deleting: {}", path.display());
        }

        match delete_dirs(paths) {
            Ok(()) => {
                println!("\nDeletion process complete.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error deleting directories: {}", e);
                Err(e)
            }
        }
    } else {
        println!("Deletion cancelled.");
        Ok(())
    }
}
