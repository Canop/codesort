use crate::*;

/// a "block" of code
#[derive(Debug, Clone)]
pub struct Block {
    start: usize,
    end: usize, // index of the first line not included
    pub base_indent: usize,
}

impl Block {
    pub fn new(
        start: usize,
        base_indent: usize,
    ) -> Self {
        Self {
            start,
            end: start + 1,
            base_indent,
        }
    }
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
    pub fn augment(&mut self) {
        self.end += 1;
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn is_end_deep(
        &self,
        list: &List,
    ) -> bool {
        for i in (0..self.end).rev() {
            if !list.lines[i].is_empty() {
                return list.lines[i].indent() > self.base_indent;
            }
        }
        false
    }
    pub fn contains_only_comments(
        &self,
        list: &List,
    ) -> bool {
        for line in &list.lines[self.start..self.end] {
            if !line.is_comment_or_attribute() {
                return false;
            }
        }
        true
    }
    /// Print the block with line numbers (for debugging)
    pub fn print(
        &self,
        list: &List,
    ) {
        for idx in self.start..self.end {
            if let Some(line) = list.lines.get(idx) {
                print!("{:>4} | {}", idx, line.content());
            }
        }
    }
    pub fn has_only_empty_lines(
        &self,
        list: &List,
    ) -> bool {
        for line in &list.lines[self.start..self.end] {
            if !line.is_empty() {
                return false;
            }
        }
        true
    }
}
