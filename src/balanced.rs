use crate::*;

/// A piece of code made of complete lines, with balanced braces
/// and some significant content
#[derive(Debug, Clone)]
pub struct Balanced {
    pub sort_key: String,
    pub is_annotation: bool,
    pub last_significant_char: Option<char>,
    pub language: Language,
}

impl Balanced {
    pub fn is_empty(&self) -> bool {
        self.last_significant_char.is_none()
    }
}
