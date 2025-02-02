use {
    codesort::*,
    include_dir::{
        Dir,
        DirEntry,
        include_dir,
    },
};

static SRC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src");

fn find_rs_files<'d>(
    dir: &'d Dir<'d>,
    files: &mut Vec<&'d include_dir::File<'d>>,
) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(sub_dir) => {
                find_rs_files(sub_dir, files);
            }
            DirEntry::File(file) => {
                if file
                    .path()
                    .extension()
                    .map(|ext| ext == "rs")
                    .unwrap_or(false)
                {
                    files.push(file);
                }
            }
        }
    }
}

/// Check that all files in the src directory are correctly "parsed"
#[test]
fn test_codesort_source_files_are_complete() {
    let mut files = Vec::new();
    find_rs_files(&SRC_DIR, &mut files);
    for file in files {
        println!("Checking {}", file.path().display());
        let input = std::str::from_utf8(file.contents()).unwrap();
        let list = LocList::read_str(input, Language::Rust).unwrap();
        assert!(list.is_complete());
    }
}
