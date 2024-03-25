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

    pub fn print_debug(
        &self,
        label: &str,
    ) {
        println!("{label:=^80}");
        for (i, loc) in self.locs.iter().enumerate() {
            println!(
                "{i:>3} {:>2}-{:<2} | {:<30}",
                loc.start_depth,
                loc.end_depth,
                loc.content.trim_end(),
            );
        }
    }
    pub fn count_blank_lines_at_start(&self) -> usize {
        self.locs.iter().take_while(|loc| loc.is_blank()).count()
    }
    pub fn has_content(&self) -> bool {
        let Some(first_start_depth) = self.locs.first().map(|loc| loc.start_depth) else {
            return false;
        };
        let mut has_not_annotation = false; // considering only root level locs
        let mut has_sortable = false;
        for loc in &self.locs {
            if loc.start_depth == first_start_depth {
                if loc.is_sortable() {
                    has_not_annotation |= !loc.is_annotation;
                }
            }
            if has_not_annotation {
                has_sortable |= loc.is_sortable();
            }
        }
        has_not_annotation && has_sortable
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
    pub fn is_complete(&self) -> bool {
        if !self.has_content() {
            return false;
        }
        let (Some(first), Some(last)) =
            (self.locs.first(), self.last_line_with_content())
        else {
            return false;
        };
        if first.start_depth != last.end_depth {
            return false;
        }
        if !last.can_complete {
            return false;
        }
        let mut wished = Vec::new();
        for loc in &self.locs {
            for gift in &loc.gifts {
                wished.retain(|&w| !gift.satisfies(w));
            }
            for wish in &loc.wishes {
                wished.push(wish);
            }
        }
        wished.is_empty()
    }
    pub fn is_complete_d(
        &self,
        debug: bool,
    ) -> bool {
        if debug {
            self.print_debug(" IS_COMPLETE ? ");
        }
        if !self.has_content() {
            if debug {
                println!("no content");
            }
            return false;
        }
        if debug {
            println!("sort key: {:?}", &self.locs.last().unwrap().sort_key);
        }
        let (Some(first), Some(last)) =
            (self.locs.first(), self.last_line_with_content())
        else {
            return false;
        };
        if first.start_depth != last.end_depth {
            if debug {
                println!("no same depth");
            }
            return false;
        }
        if !last.can_complete {
            if debug {
                println!("no can compete");
            }
            return false;
        }
        let mut wished = Vec::new();
        for loc in &self.locs {
            for gift in &loc.gifts {
                wished.retain(|&w| !gift.satisfies(w));
            }
            for wish in &loc.wishes {
                wished.push(wish);
            }
        }
        if debug {
            println!("wished empty? {}", wished.is_empty());
        }
        wished.is_empty()
    }
    pub fn into_blocks(self) -> Vec<LocList> {
        let mut blocs = Vec::new();
        let mut current = LocList::default();
        let mut debug = false;
        for loc in self.locs {
            current.locs.push(loc);
            if current.is_complete_d(debug) {
                blocs.push(std::mem::take(&mut current));
                debug = false;
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
