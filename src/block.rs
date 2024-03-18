use crate::*;

/// a "block" of code, for a given list
#[derive(Debug, Clone)]
pub struct Block {
    pub code: String,
    pub balanced: Option<Balanced>,
    pub start: usize,
    pub end: usize, // index of the first line not included
    pub base_indent: usize,
}

impl Block {
    pub fn new(
        start: usize,
        list: &List,
    ) -> Self {
        let line = &list.lines[start];
        let code = line.content().to_string();
        let balanced = list.lang.check_balanced(&code);
        Self {
            code,
            balanced,
            start,
            end: start + 1,
            base_indent: line.indent(),
        }
    }
    pub fn content(&self) -> &str {
        &self.code
    }
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
    pub fn augment(
        &mut self,
        list: &List,
    ) {
        let line = &list.lines[self.end];
        self.code.push_str(line.content());
        self.balanced = list.lang.check_balanced(&self.code);
        self.end += 1;
    }
    pub fn is_balanced(&self) -> bool {
        self.balanced.is_some()
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// is the block an annotation which should be merged with
    /// a following block (unless it's the last one)
    ///
    pub fn is_annotation(&self) -> bool {
        self.balanced.as_ref().map_or(false, |b| b.is_annotation)
    }
    pub fn is_complete(&self) -> bool {
        match self.balanced {
            Some(ref balanced) => match balanced.last_significant_char {
                Some(';' | '}' | ',' | ']') => true,
                _ => false,
            },
            None => false,
        }
    }
    /// the key to use for sorting
    ///
    /// If the block is empty, it should be at the end (it was originally
    /// at the end or it would have been merged during block extraction)
    pub fn sort_key(&self) -> &str {
        self.balanced
            .as_ref()
            .filter(|b| !b.is_empty())
            .map_or("~~", |b| &b.sort_key)
    }
}
