use std::io;

mod modules;

use modules::deleting_dirs::handle_delete_dirs;
use modules::getting_path::handle_getting_working_path;
use modules::searching_dirs::handle_searching_dirs;

fn main() -> io::Result<()> {
    println!("oRganizer");

    let base_path = handle_getting_working_path()?;
    let selected_folders = handle_searching_dirs(base_path)?;
    handle_delete_dirs(selected_folders)?;

    Ok(())
}
