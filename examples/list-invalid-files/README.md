

This tool looks at all rust files in a given path, and checks that all of them are correctly analyzed.

If some file appears not complete, sort calls would be refused. Those files are listed.

Usage:

```bash
cargo run --release --example list-invalid-files -- ../rustlang/rust
```
