use crate::*;

/// a "block" of code
#[derive(Debug, Clone)]
pub struct Block {
    code: String,
    balanced: Option<Balanced>,
    start: usize,
    end: usize, // index of the first line not included
    pub base_indent: usize,
}

impl Block {
    pub fn new(
        start: usize,
        list: &List,
    ) -> Self {
        let line = &list.lines[start];
        let code = line.content().to_string();
        let balanced = Balanced::new(&code);
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
        self.balanced = Balanced::new(&self.code);
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
    pub fn is_complete(&self) -> bool {
        match self.balanced {
            Some(ref balanced) => match balanced.last_significant_char {
                ';' | '}' | ',' => true,
                _ => false,
            },
            None => false,
        }
    }
}
