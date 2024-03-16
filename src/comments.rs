/// Try to trim end of line comments without parsing
/// the code, so as to work in most cases and most languages
/// of the C family (hard to make a correct expression checker
/// which accepts rust raw strings, single quoted js strings,
/// rust lifetimes, etc.).
///
/// This function isn't used to check block balancing or
/// completeness, but only to determine what part of the line
/// should be used for sorting.
pub fn trim_comments(s: &str) -> &str {
    let mut in_string = false;
    let mut last_is_antislash = false;
    for (i, c) in s.char_indices() {
        if c == '"' && !last_is_antislash {
            in_string = !in_string;
        }
        if !in_string && !last_is_antislash && s[i..].starts_with("//") {
            return &s[..i];
        }
        last_is_antislash = c == '\\';
    }
    s
}
