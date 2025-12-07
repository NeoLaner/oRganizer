use std::{
    io::{self, stdin},
    path::PathBuf,
};

/// Pure validation function - no I/O, just logic
fn validate_path(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!(
            "The specified path '{}' does not exist.",
            path.display()
        ));
    }
    if !path.is_dir() {
        return Err(format!(
            "The specified path '{}' is not a directory.",
            path.display()
        ));
    }
    Ok(())
}

/// Gets user input and returns the trimmed path string
fn get_path_input() -> io::Result<String> {
    let mut path_input = String::new();
    stdin().read_line(&mut path_input)?;
    Ok(path_input.trim().to_string())
}

/// Handles the interactive prompt loop - manages UI/printing
pub fn handle_getting_working_path() -> io::Result<PathBuf> {
    loop {
        println!("Enter the base directory path to search (e.g., . for current directory):");

        let input = get_path_input()?;
        let path = PathBuf::from(input);

        match validate_path(&path) {
            Ok(()) => return Ok(path),
            Err(error_msg) => {
                eprintln!("Error: {} Please try again.", error_msg);
            }
        }
    }
}
