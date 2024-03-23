use {
    crate::*,
    std::{
        cmp::Ordering,
        fmt,
    },
};

/// A Line of Code, analyzed.
///
/// While public, this is implementation dependant and may change
/// on patch versions.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Loc {
    /// The whole content of the line, including the ending newline
    pub content: String,
    /// The key used for sorting, may be empty
    pub sort_key: String,
    /// number of bytes of leading spaces
    pub indent: usize,
    /// The syntactic depth considered on the whole file, at start of line
    pub start_depth: usize,
    /// The syntactic depth considered on the whole file, at end of line
    pub end_depth: usize,
    /// Whether this line starts a java annotation, a rust attribute, etc.
    pub is_annotation: bool,
    pub can_complete: bool,
    /// wishes needed after this loc
    pub wishes: Vec<Wish>,
    /// gifts not required by this loc
    pub gifts: Vec<Gift>,
}

impl Loc {
    /// Either the depth at start, or the depth at end, whichever is smaller
    pub fn min_depth(&self) -> usize {
        self.start_depth.min(self.end_depth)
    }
    /// Whether the deindented content starts with the given string
    pub fn starts_with(
        &self,
        s: &str,
    ) -> bool {
        self.content[self.indent..].starts_with(s)
    }
    /// the last character which is not a whitespace or part of a comment
    pub fn last_significant_char(&self) -> Option<char> {
        self.sort_key.chars().rev().find(|c| !c.is_whitespace())
    }
    pub fn is_blank(&self) -> bool {
        !self.content[self.indent..]
            .chars()
            .any(|c| !c.is_whitespace())
    }
    pub fn is_sortable(&self) -> bool {
        !self.is_annotation && !self.sort_key.is_empty()
    }
}

impl PartialEq for Loc {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.sort_key == other.sort_key
    }
}
impl Eq for Loc {}
impl Ord for Loc {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.sort_key.cmp(&other.sort_key)
    }
}
impl PartialOrd for Loc {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Loc {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", &self.content)
    }
}
