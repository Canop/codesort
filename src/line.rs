use {
    crate::*,
    std::fmt,
};

#[derive(Debug, Clone)]
pub struct Line {
    content: String,
    /// Number of bytes of leading spaces and tabs
    indent: usize,
    /// number of bytes from the start to the end of the real content
    /// (excluding trailing spaces and comments)
    inner_end: usize,
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
        let inner_end = trim_comments(&content[indent..]).trim_end().len() + indent;
        Line {
            content,
            indent,
            inner_end,
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn inner(&self) -> &str {
        &self.content[self.indent..self.inner_end]
    }
    //pub fn deindent(&self) -> &str {
    //    &self.content[self.indent..]
    //}
    pub fn starts_with(
        &self,
        s: &str,
    ) -> bool {
        self.inner().starts_with(s)
    }
    pub fn ends_with(
        &self,
        s: &str,
    ) -> bool {
        self.inner().ends_with(s)
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
    pub fn is_comment_or_attribute(&self) -> bool {
        self.indent == self.inner_end
            || self.starts_with("#") // comment in some languages, attribute in rust
            || self.starts_with("@") // Java annotation
    }
    pub fn is_opening(&self) -> bool {
        self.ends_with("{") || self.ends_with("(")
    }
    pub fn is_closing(&self) -> bool {
        self.starts_with("}") || self.starts_with(")") || self.ends_with(",")
    }
    pub fn can_extend(
        &self,
        indent: usize,
    ) -> bool {
        self.indent() >= indent || self.is_empty()
    }
    /// Returns true if the line is empty, a comment, a doc comment,
    /// a rust attribute, etc.
    ///
    /// Those lines must be ignored when sorting
    pub fn exclude_from_sort(&self) -> bool {
        self.is_empty()
            || self.indent == self.inner_end // contains only line-end comments
            || self.starts_with("#") // comment in some languages, attribute in rust
            || self.starts_with("@") // Java annotation
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
