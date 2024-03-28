

Give this program a path, and it sorts the enums it finds in all rust files.

When some attributes are found in a file, it's not modified, because enum variants order is supposed to matter: `repr`, `serde(Other)` or `serde(untagged)`.

Example:

```bash
cargo run --release --example sort-all-enums path/to/project
```

Of course, sorting all enums of a codebase is at best useless, and most probably a nuisance because most enums are better sorted another way.
The real goal is to check whether there are rust files that codesort fails to anayze or fails to sort.
