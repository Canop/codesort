use {
    clap::Parser,
    codesort::*,
    lazy_regex::*,
    std::{
        fs,
        io,
        path::PathBuf,
    },
    termimad::crossterm::style::Stylize,
};

static EXCLUDED_DIRS: &[&str] = &[".git", "target", "build"];

/// Sort all enums of all rust files found in the given directory
///
/// Are excluded
/// - files in .git, target and build directories
/// - files which don't appear correct enough
#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Args {
    /// directories normally excluded to include
    #[clap(long, default_value = "")]
    pub include: Vec<String>,

    /// Path to the file(s)
    pub path: PathBuf,
}

pub fn get_all_rust_files(
    root: PathBuf,
    include: &[String],
) -> io::Result<Vec<PathBuf>> {
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
                if file_name.starts_with('.') {
                    continue;
                }
                if EXCLUDED_DIRS.contains(&file_name) {
                    if !include.iter().any(|inc| inc == file_name) {
                        eprintln!("{} {:?}", "Excluded".yellow(), path);
                        continue;
                    }
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

fn main() -> CsResult<()> {
    let args = Args::parse();
    let files = get_all_rust_files(args.path, &args.include)?;
    eprintln!("Found {} rust files", files.len());
    let mut enum_count = 0;
    let mut not_complete_count = 0;
    let mut ok_count = 0;
    for file in &files {
        let loc_list = LocList::read_file(file, Language::Rust);
        let mut loc_list = match loc_list {
            Ok(loc_list) => loc_list,
            Err(e) => {
                eprintln!("{} in {}: {:?}", "ERROR".red(), file.display(), e);
                continue;
            }
        };
        if !loc_list.has_content() {
            continue;
        }
        if !loc_list.is_complete() {
            eprintln!("skipping {} (not consistent enough)", file.display());
            not_complete_count += 1;
            continue;
        }
        let mut modified = false;
        let mut line_idx = 0;
        ok_count += 1;
        while line_idx + 2 < loc_list.len() {
            let loc = &loc_list.locs[line_idx];
            let content = &loc.content;
            let Some((_, name)) =
                regex_captures!(r"^[\s\w()]*\benum\s+([^({]+)\s+\{\s**$", content)
            else {
                line_idx += 1;
                continue;
            };
            let range = loc_list.range_around_line_index(line_idx + 1).unwrap();
            eprintln!("Sorting enum {}", name.blue());
            loc_list.sort_range(range).unwrap();
            line_idx = range.end.to_index() + 2;
            enum_count += 1;
            modified = true;
        }
        if modified {
            loc_list.write_file(file)?;
            eprintln!("wrote {}", file.display());
        }
        if enum_count >= 150 {
            eprintln!("stop");
            break;
        }
    }
    eprintln!("I sorted {} enums in {} files", enum_count, ok_count);
    eprintln!("I encountered {} not complete files", not_complete_count);
    Ok(())
}
