use {
    crate::*,
    std::{
        fmt,
        str::FromStr,
    },
};

#[derive(Debug, Clone)]
pub struct List {
    pub lines: Vec<Line>,
}

impl FromStr for List {
    type Err = CsError;
    fn from_str(s: &str) -> CsResult<Self> {
        Self::from_reader(s.as_bytes())
    }
}

impl List {
    pub fn from_bytes(bytes: &[u8]) -> CsResult<Self> {
        Self::from_reader(bytes)
    }
    pub fn from_reader<R: std::io::BufRead>(mut reader: R) -> CsResult<Self> {
        let mut lines = Vec::new();
        loop {
            let mut content = String::new();
            let n = reader.read_line(&mut content)?;
            if n == 0 {
                break;
            }
            lines.push(Line::from(content));
        }
        Ok(List { lines })
    }
    /// Determine the biggest possible range around a line index
    ///
    /// Return an empty window if the line index is out of bounds
    pub fn window_around(
        self,
        line_idx: usize,
    ) -> Window {
        let mut start = line_idx;
        let mut end = line_idx;
        if line_idx < self.lines.len() {
            let indent = self.lines[line_idx].indent();
            while start > 0 && self.lines[start - 1].can_extend(indent) {
                start -= 1;
            }
            while end < self.lines.len() - 1 && self.lines[end + 1].can_extend(indent) {
                end += 1;
            }
            end += 1;
        }
        Window {
            list: self,
            start,
            end,
        }
    }
    pub fn window_on_range(
        self,
        start: usize,
        end: usize,
    ) -> Window {
        Window {
            list: self,
            start,
            end,
        }
    }
    pub fn into_window(self) -> Window {
        let end = self.lines.len();
        Window {
            list: self,
            start: 0,
            end,
        }
    }
    pub fn first_not_empty_line_after(
        &self,
        line_idx: usize,
    ) -> Option<usize> {
        for i in line_idx + 1..self.lines.len() {
            if !self.lines[i].is_empty() {
                return Some(i);
            }
        }
        None
    }
}

impl fmt::Display for List {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for line in &self.lines {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}
