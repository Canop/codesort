# An example script sorting most enums of the rustlang/rust project
# 
# Right now, there are more excluded directories than what I'd like,
# probably because of the transmute, but this is a call for more work...
cargo run --example sort-all-enums \
    -- \
    --exclude "test" \
    --exclude "test_data" \
    --exclude "ops" \
    --exclude "compiler" \
    --exclude "rustc_hir_typeck" \
    --exclude "rustc_builtin_macros" \
    --exclude "rustc_lint" \
    --exclude "rustc_lint_defs" \
    --exclude "rustc_macros" \
    --exclude "rustc_middle" \
    --exclude "rustc_resolve" \
    --exclude "rustc_session" \
    ~/dev/rust
