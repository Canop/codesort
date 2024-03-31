

Give this program a path, and it sorts the enums it finds in all rust files.

Example:

```bash
cargo run --release --example sort-all-enums ~/dev/rustlang/rust
```

When some attributes are found in a file, it's not modified, because enum variants order may matter.
Some folders also may contain rust files which should not be touched.

Here are the standard exclusions:

```
/// Directories we don't want to touch
static EXCLUDED_DIRS: &[&str] = &[".git", "target", "build"];

/// Keywords which, if found in the annotations before an enum, will
/// prevent the enum variants from being sorted
static EXCLUDING_KEYWORDS: &[&str] = &[
    "repr",
    "serde",
    "PartialOrd", "Ord",
];
```

Of course, sorting all enums of a codebase is at best useless, and most probably a nuisance because most enums are better sorted another way.
The real goal is to check whether there are rust files that codesort fails to anayze or fails to sort.

IMO it is a success: the rustlang/rust codebase compiles after having all enums passing the filters being sorted.

