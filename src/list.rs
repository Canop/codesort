use {
    crate::*,
    std::{
        fmt,
        str::FromStr,
    },
};

/// A text, as a list of lines
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
    pub fn inner_line_by_number(
        &self,
        line_number: LineNumber,
    ) -> Option<&str> {
        self.lines
            .get(line_number.to_index())
            .map(|line| line.inner())
    }
    pub fn line_by_number(
        &self,
        line_number: LineNumber,
    ) -> Option<&str> {
        self.lines
            .get(line_number.to_index())
            .map(|line| line.content())
    }
    fn has_not_empty_line_at(
        &self,
        line_idx: usize,
    ) -> bool {
        line_idx < self.lines.len() && !self.lines[line_idx].is_empty()
    }
    pub fn non_empty_line_around(
        &self,
        line_idx: LineIndex,
    ) -> Option<usize> {
        if self.has_not_empty_line_at(line_idx) {
            return Some(line_idx);
        }
        if self.has_not_empty_line_at(line_idx + 1) {
            return Some(line_idx);
        }
        if self.has_not_empty_line_at(line_idx - 1) {
            return Some(line_idx);
        }
        if self.has_not_empty_line_at(line_idx + 2) {
            return Some(line_idx);
        }
        if self.has_not_empty_line_at(line_idx - 2) {
            return Some(line_idx);
        }
        None
    }
    /// Determine the biggest possible range around a line
    ///
    /// (takes a 1-based line number)
    pub fn window_around_line(
        self,
        line_number: LineNumber,
    ) -> CsResult<Window> {
        Self::window_around(self, line_number.to_index())
    }
    /// Determine the biggest possible range around a line index
    ///
    /// (takes a 0-based line index)
    pub fn window_around(
        self,
        line_idx: LineIndex,
    ) -> CsResult<Window> {
        let Some(line_idx) = self.non_empty_line_around(line_idx) else {
            return Err(CsError::NoSortableRangeAround(line_idx));
        };
        let mut start = line_idx;
        let mut end = line_idx;
        let indent = self.lines[line_idx].indent();
        while start > 0 && self.lines[start - 1].can_extend(indent) {
            start -= 1;
        }
        while end < self.lines.len() - 1 && self.lines[end + 1].can_extend(indent) {
            end += 1;
        }
        end += 1;
        Ok(Window {
            list: self,
            start,
            end,
        })
    }
    /// Build a window on a range of 0-based line indices,
    /// end not included
    pub fn window_on_range(
        self,
        start: LineIndex,
        end: LineIndex,
    ) -> CsResult<Window> {
        if end <= start {
            return Err(CsError::InvalidRange { start, end });
        }
        Ok(Window {
            list: self,
            start,
            end,
        })
    }
    /// Build a window on a range
    pub fn window_on_line_range(
        self,
        range: LineNumberRange,
    ) -> CsResult<Window> {
        Self::window_on_range(self, range.start.to_index(), range.end.to_index() + 1)
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
    /// Print the lines, for debug
    pub fn tprint(&self) {
        for (i, line) in self.lines.iter().enumerate() {
            print!("{:>3} | {}", i + 1, line.content());
        }
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
