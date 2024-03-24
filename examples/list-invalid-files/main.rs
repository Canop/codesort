use {
    clap::Parser,
    codesort::*,
    std::{
        fs,
        io,
        path::PathBuf,
    },
    termimad::crossterm::style::Stylize,
};

/// Launch arguments
#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Args {
    /// Path to the file(s)
    pub path: Option<PathBuf>,
}

pub fn get_all_rust_files(root: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    // if we're given a single file, it's probably because the user
    // wants to sort it, so we don't check the extension
    if !root.is_dir() {
        files.push(root);
        return Ok(files);
    }
    let mut dirs = vec![root];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            let Some(file_name) = path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if path.is_dir() {
                if file_name.starts_with('.') || file_name == "target" {
                    // in a more serious program, we would check .gitignore
                    continue;
                }
                dirs.push(path.to_path_buf());
                continue;
            }
            if let Some(ext) = path.extension() {
                if ext.to_str() == Some("rs") {
                    files.push(path.to_path_buf());
                }
            }
        }
    }
    Ok(files)
}

fn main() {
    let args = Args::parse();
    let files = get_all_rust_files(args.path.unwrap()).unwrap();
    eprintln!("Found {} rust files", files.len());
    let mut no_complete_count = 0;
    let mut errors = 0;
    let mut ok_count = 0;
    for file in files {
        let loc_list = LocList::read_file(file.to_str().unwrap(), Language::Rust);
        let loc_list = match loc_list {
            Ok(loc_list) => loc_list,
            Err(e) => {
                eprintln!("{} {:?} : {}", "ERROR".red(), file, e);
                errors += 1;
                continue;
            }
        };
        if loc_list.is_complete() {
            ok_count += 1;
            continue;
        }
        if !loc_list.has_content() {
            eprintln!("{} {:?}", "EMPTY".yellow(), file);
            ok_count += 1;
            continue;
        }
        eprintln!("{} {:?}", "NOT COMPLETE".yellow(), file);
        no_complete_count += 1;
    }
    eprintln!("OK files: {}", ok_count);
    eprintln!("Erroring files: {}", errors);
    eprintln!("Uncomplete files: {}", no_complete_count);
}
