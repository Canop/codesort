

Give this program a path, and it sorts all enums it finds in all rust files.

Example:

```bash
cargo run --release --example sort-all-enums ~/dev/rustlang/rust
```

Of course, sorting all enums of a codebase is at best useless, and most probably a nuisance because most enums are better sorted another way.
The real goal is to check whether there are rust files that codesort fails to anayze or fails to sort.
