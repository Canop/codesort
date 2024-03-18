use std::fmt;

#[derive(Debug, Clone)]
pub struct Line {
    content: String,
    /// Number of bytes of leading spaces and tabs
    indent: usize,
}

impl From<String> for Line {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}
impl From<&str> for Line {
    fn from(content: &str) -> Self {
        Self::from(content.to_string())
    }
}

impl Line {
    pub fn new(content: String) -> Self {
        let indent = content
            .as_bytes()
            .iter()
            .take_while(|&&b| b == b' ' || b == b'\t')
            .count();
        Line { content, indent }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn deindent(&self) -> &str {
        &self.content[self.indent..]
    }
    pub fn starts_with(
        &self,
        s: &str,
    ) -> bool {
        self.deindent().starts_with(s)
    }
    pub fn indent(&self) -> usize {
        self.indent
    }
    pub fn into_content(self) -> String {
        self.content
    }
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
    pub fn can_extend(
        &self,
        indent: usize,
    ) -> bool {
        self.indent() >= indent || self.is_empty()
    }
}

impl fmt::Display for Line {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}
