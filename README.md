# oRganizer

A simple Rust command-line tool that helps you find and delete `node_modules` directories in your projects to free up disk space.

## Features

- Recursively searches for `node_modules` directories from a specified path
- Shows all found directories before deletion
- Requires confirmation before deleting any directories
- Safe error handling for file system operations

## Usage

1. Run the program
2. Enter the base directory path to search (e.g., `.` for current directory or drag & drop folder to cli)
3. Review the list of found `node_modules` directories
4. Confirm deletion by typing 'y' (or any other key to cancel)

## Building from Source

```bash
git clone https://github.com/yourusername/orgonizer.git
cd orgonizer
cargo build --release
```

## Running the Program

```bash
cargo run
```

## License

MIT License
