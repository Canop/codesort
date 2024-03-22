use {
    crate::*,
    std::{
        cmp::Ordering,
        fmt,
    },
};

#[derive(Debug, Clone)]
pub struct Loc {
    pub content: String,
    pub sort_key: String,
    /// number of bytes of leading spaces
    pub indent: usize,
    pub start_depth: usize,
    pub end_depth: usize,
    pub is_annotation: bool,
    pub can_complete: bool,
    pub wishes: Vec<Wish>, // wishes needed after this loc
    pub gifts: Vec<Gift>,  // gifts not required by this loc
}

impl Loc {
    pub fn min_depth(&self) -> usize {
        self.start_depth.min(self.end_depth)
    }
    pub fn starts_with(
        &self,
        s: &str,
    ) -> bool {
        self.content.trim_start().starts_with(s)
    }
    pub fn last_significant_char(&self) -> Option<char> {
        self.sort_key.chars().rev().find(|c| !c.is_whitespace())
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
