use {
    crate::*,
    std::{
        cmp::Ordering,
        fmt,
        fs,
        path::Path,
    },
};

/// A list of Lines of Code
///
/// To sort it, you focus it, which specifies the area to sort, then
/// you call sort it.
#[derive(Debug, Clone, Default)]
pub struct LocList {
    pub locs: Vec<Loc>,
}

impl LocList {
    pub fn read<R: std::io::BufRead>(
        reader: R,
        lang: Language,
    ) -> CsResult<Self> {
        lang.analyzer().read(reader)
    }
    pub fn read_str(
        s: &str,
        lang: Language,
    ) -> CsResult<LocList> {
        Self::read(s.as_bytes(), lang)
    }
    pub fn read_file<P: AsRef<Path>>(
        path: P,
        lang: Language,
    ) -> CsResult<LocList> {
        let s = fs::read_to_string(path)?;
        Self::read(s.as_bytes(), lang)
    }
    pub fn write_file<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> CsResult<()> {
        fs::write(path, self.to_string())?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.locs.len()
    }

    pub fn sort_range(
        &mut self,
        range: LineNumberRange,
    ) -> CsResult<()> {
        let list = LocList {
            locs: std::mem::take(&mut self.locs),
        };
        let focused = list.focus(range)?;
        let sorted = focused.sort();
        self.locs = sorted.locs;
        Ok(())
    }
    pub fn sort_around_line_index(
        &mut self,
        line_index: LineIndex,
    ) -> CsResult<()> {
        let range = self.range_around_line_index(line_index)?;
        self.sort_range(range)
    }
    pub fn sort_around_line_number(
        &mut self,
        line_number: LineNumber,
    ) -> CsResult<()> {
        let range = self.range_around_line_number(line_number)?;
        self.sort_range(range)
    }

    pub fn focus_all(self) -> CsResult<Focused> {
        Ok(Focused {
            before: LocList::default(),
            focus: self.clone(),
            after: LocList::default(),
        })
    }
    pub fn focus(
        mut self,
        range: LineNumberRange,
    ) -> CsResult<Focused> {
        let start = range.start.to_index();
        let end = range.end.to_index();
        if start >= self.locs.len() || end >= self.locs.len() {
            return Err(CsError::InvalidRange { start, end });
        }
        let focus = LocList {
            locs: self.locs.drain(start..=end).collect(),
        };
        let before = LocList {
            locs: self.locs.drain(..start).collect(),
        };
        let after = LocList {
            locs: self.locs.drain(..).collect(),
        };
        Ok(Focused {
            before,
            focus,
            after,
        })
    }
    pub fn focus_around_line_index(
        self,
        line_idx: LineIndex,
    ) -> CsResult<Focused> {
        let range = self.range_around_line_index(line_idx)?;
        self.focus(range)
    }
    pub fn focus_around_line_number(
        self,
        line_number: LineNumber,
    ) -> CsResult<Focused> {
        let range = self.range_around_line_index(line_number.to_index())?;
        self.focus(range)
    }
    pub fn line_at_number(
        &self,
        line_number: LineNumber,
    ) -> Option<&Loc> {
        self.locs.get(line_number.to_index())
    }
    pub fn print_range_debug(
        &self,
        label: &str,
        range: LineNumberRange,
    ) {
        println!("{label:=^80}");
        for ln in range {
            let loc = self.line_at_number(ln);
            match loc {
                Some(loc) => {
                    println!(
                        "{:>4} {:>2}-{:<2} | {:<30}",
                        ln,
                        loc.start_depth,
                        loc.end_depth,
                        loc.content.trim_end(),
                    );
                }
                None => {
                    println!("{:>3} | <no loc>", ln);
                }
            }
        }
    }
    pub fn print_debug(
        &self,
        label: &str,
    ) {
        let Some(range) = self.full_range_checked() else {
            println!("{}: <empty>", label);
            return;
        };
        self.print_range_debug(label, range);
    }
    pub fn trimmed_range(
        &self,
        mut range: LineNumberRange,
    ) -> LineNumberRange {
        while range.start < range.end && self.locs[range.start.to_index()].is_blank() {
            range.start = LineNumber::from_index(range.start.to_index() + 1);
        }
        while range.end > range.start && self.locs[range.end.to_index()].is_blank() {
            range.end = LineNumber::from_index(range.end.to_index() - 1);
        }
        range
    }
    pub fn count_blank_lines_at_start(&self) -> usize {
        self.locs.iter().take_while(|loc| loc.is_blank()).count()
    }
    pub fn full_range(&self) -> LineNumberRange {
        self.full_range_checked().unwrap()
    }
    pub fn full_range_checked(&self) -> Option<LineNumberRange> {
        if self.locs.is_empty() {
            None
        } else {
            Some(LineNumberRange {
                start: LineNumber::from_index(0),
                end: LineNumber::from_index(self.locs.len() - 1),
            })
        }
    }
    pub fn check_range(
        &self,
        range: LineNumberRange,
    ) -> CsResult<()> {
        let start = range.start.to_index();
        let end = range.end.to_index();
        if start >= self.locs.len() || end >= self.locs.len() || start > end {
            return Err(CsError::InvalidRange { start, end });
        }
        Ok(())
    }
    /// Check whether the range is valid and contains at least one line
    pub fn range_exists(
        &self,
        range: LineNumberRange,
    ) -> bool {
        let start = range.start.to_index();
        let end = range.end.to_index();
        start < self.locs.len() && end < self.locs.len() && start <= end
    }
    pub fn range_has_content(
        &self,
        range: LineNumberRange,
    ) -> bool {
        if !self.range_exists(range) {
            return false;
        }
        let first_start_depth = self.locs[range.start.to_index()].start_depth;
        let mut has_not_annotation = false; // considering only root level locs
        let mut has_sortable = false;
        for loc in &self.locs[range.start.to_index()..=range.end.to_index()] {
            if loc.start_depth == first_start_depth {
                if loc.is_sortable() {
                    has_not_annotation |= !loc.is_annotation;
                }
            }
            if has_not_annotation {
                has_sortable |= loc.is_sortable();
            }
            if has_not_annotation && has_sortable {
                return true;
            }
        }
        false
    }
    pub fn has_content(&self) -> bool {
        self.full_range_checked()
            .map_or(false, |range| self.range_has_content(range))
    }
    pub fn last_significant_char(&self) -> Option<char> {
        self.locs
            .iter()
            .rev()
            .find_map(|loc| loc.last_significant_char())
    }
    pub fn last_line_with_content(&self) -> Option<&Loc> {
        self.locs.iter().rev().find(|&loc| loc.is_sortable())
    }
    pub fn last_line_in_range_with_content(
        &self,
        range: LineNumberRange,
    ) -> Option<&Loc> {
        self.locs[range.start.to_index()..=range.end.to_index()]
            .iter()
            .rev()
            .find(|&loc| loc.is_sortable())
    }
    /// Warning: doesn't check whether a bigger range wouldn't be
    /// complete too, so a range being complete doesn't mean it is
    /// a block.
    pub fn is_range_complete(
        &self,
        range: LineNumberRange,
    ) -> bool {
        if !self.range_has_content(range) {
            return false;
        }
        let first = &self.locs[range.start.to_index()];
        let Some(last) = self.last_line_in_range_with_content(range) else {
            return false;
        };
        if first.start_depth != last.end_depth {
            return false;
        }
        if !last.can_complete {
            return false;
        }
        let mut wished = Vec::new();
        for loc in &self.locs[range.start.to_index()..=range.end.to_index()] {
            for gift in &loc.gifts {
                wished.retain(|&w| !gift.satisfies(w));
            }
            for wish in &loc.wishes {
                wished.push(wish);
            }
        }
        wished.is_empty()
    }
    pub fn is_complete(&self) -> bool {
        self.is_range_complete(self.full_range())
    }
    /// Assuming the provided range you pass is valid enough, give the ranges of the
    /// blocks in it.
    pub fn block_ranges_in_range(
        &self,
        range: LineNumberRange,
    ) -> Vec<LineNumberRange> {
        let mut blocks = Vec::new();
        let mut current = LineNumberRange::of_line(range.start);
        for line_number in range {
            if self.is_range_complete(current) {
                blocks.push(current);
                current = LineNumberRange::of_line(line_number);
            } else {
                current.end = line_number;
            }
        }
        if blocks.last().map_or(true, |last| last.end != range.end) {
            blocks.push(current);
        }
        blocks
    }
    /// Find the block the line is part of
    ///
    /// For example, if you give the opening line of a struct
    /// (i.e a like like `pub struct Foo {`), you'll get
    /// the whole struct with its doc comments, annotations, inner
    /// fields, til the closing brace. You also get the empty lines
    /// before which should stick with the block.
    ///
    /// If you give the line of a field, you'll get the field definition
    /// with its own comment and annotation.
    pub fn block_range_of_line_number(
        &self,
        line_number: LineNumber,
    ) -> CsResult<LineNumberRange> {
        let range = self.range_around_line_number(line_number)?;
        let blocks = self.block_ranges_in_range(range);
        for block in blocks {
            if block.contains(line_number) {
                return Ok(block);
            }
        }
        Err(CsError::InvalidRange {
            // should never happen
            start: range.start.to_index(),
            end: range.end.to_index(),
        })
    }
    pub fn into_blocks(self) -> Vec<LocList> {
        let mut blocs = Vec::new();
        let mut current = LocList::default();
        for loc in self.locs {
            current.locs.push(loc);
            if current.is_complete() {
                blocs.push(std::mem::take(&mut current));
            }
        }
        if !current.locs.is_empty() {
            blocs.push(current);
        }
        blocs
    }
    pub fn range_around_line_number(
        &self,
        line_number: LineNumber,
    ) -> CsResult<LineNumberRange> {
        self.range_around_line_index(line_number.to_index())
    }
    pub fn range_around_line_index(
        &self,
        line_idx: LineIndex,
    ) -> CsResult<LineNumberRange> {
        let locs = &self.locs;
        if line_idx >= locs.len() {
            return Err(CsError::NoSortableRangeAround(line_idx));
        }
        let mut start = line_idx;
        let mut end = line_idx;
        while start > 0 && locs[start - 1].min_depth() >= locs[line_idx].min_depth() {
            start -= 1;
        }
        while end < locs.len() - 1
            && locs[end + 1].min_depth() >= locs[line_idx].min_depth()
        {
            end += 1;
        }
        // we remove the trailing empty lines or comments: they should stick with the
        //  end of the container
        while end > line_idx && !locs[end].is_sortable() {
            end -= 1;
        }
        Ok(LineNumberRange {
            start: LineNumber::from_index(start),
            end: LineNumber::from_index(end),
        })
    }
}

impl fmt::Display for LocList {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for loc in &self.locs {
            write!(f, "{}", loc)?;
        }
        Ok(())
    }
}

impl PartialEq for LocList {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        let mut ia = 0;
        let mut ib = 0;
        while ia < self.locs.len() && ib < other.locs.len() {
            let a = &self.locs[ia];
            let b = &other.locs[ib];
            if a != b {
                return false;
            }
            ia += 1;
            ib += 1;
        }
        ia == ib
    }
}
impl Eq for LocList {}
impl Ord for LocList {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        let mut ia = 0;
        let mut ib = 0;
        loop {
            while ia < self.locs.len() && !self.locs[ia].is_sortable() {
                ia += 1;
            }
            while ib < other.locs.len() && !other.locs[ib].is_sortable() {
                ib += 1;
            }
            match (ia < self.locs.len(), ib < other.locs.len()) {
                (true, false) => return Ordering::Greater,
                (false, true) => return Ordering::Less,
                (false, false) => return Ordering::Equal,
                _ => (),
            }
            let order = self.locs[ia].cmp(&other.locs[ib]);
            if order != Ordering::Equal {
                return order;
            }
            ia += 1;
            ib += 1;
        }
    }
}
impl PartialOrd for LocList {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
