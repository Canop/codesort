use {
    crate::*,
    std::{
        cmp::Ordering,
        io::Write,
    },
};

#[derive(Debug, Clone)]
pub struct Loc {
    pub content: String,
    pub sort_key: String,
    /// number of bytes of leading spaces
    pub indent: usize,
    pub start_depth: usize,
    pub end_depth: usize,
    pub braces_at_end: BraceStack,
    pub is_annotation: bool,
    pub can_complete: bool,
}
#[derive(Debug, Clone, Default)]
pub struct LocList {
    pub locs: Vec<Loc>,
}
#[derive(Debug, Clone)]
pub struct Focused {
    pub before: LocList,
    pub focus: LocList,
    pub after: LocList,
}


impl Loc {
    pub fn min_depth(&self) -> usize {
        self.start_depth.min(self.end_depth)
    }
    pub fn starts_with(&self, s: &str) -> bool {
        self.content.trim_start().starts_with(s)
    }
    pub fn last_significant_char(&self) -> Option<char> {
        self.sort_key.chars().rev().find(|c| !c.is_whitespace())
    }
    pub fn is_sortable(&self) -> bool {
        !self.is_annotation && !self.sort_key.is_empty()
    }
}

impl LocList {
    pub fn focus(mut self, range: LineNumberRange) -> CsResult<Focused> {
        let start = range.start.to_index();
        let end = range.end.to_index();
        if start >= self.locs.len() || end >= self.locs.len() {
            return Err(CsError::InvalidRange{start, end});
        }
        let focus = LocList { locs: self.locs.drain(start..=end).collect()};
        let before = LocList { locs: self.locs.drain(..start).collect()};
        let after = LocList { locs: self.locs.drain(..).collect()};
        Ok(Focused {
            before,
            focus,
            after,
        })
    }
    pub fn print_debug(&self, label: &str) -> () {
        println!("{label:=^80}");
        for (i, loc) in self.locs.iter().enumerate() {
            println!(
                "{i:>3} {:>2}-{:<2} | {} | {:<30}",
                loc.start_depth,
                loc.end_depth,
                loc.last_significant_char().unwrap_or(' '),
                loc.content.trim_end(),
            );
        }
    }
    pub fn is_complete(&self) -> bool {
        let is_empty = self.locs.iter()
            .all(|loc| loc.is_annotation || loc.sort_key.is_empty());
        if is_empty {
            return false;
        }
        match (self.locs.first(), self.locs.last()) {
            (Some(first), Some(last)) => {
                first.start_depth == last.end_depth
                && last.can_complete
            }
            _ => false,
        }
    }
    pub fn into_blocks(self) -> Vec<LocList> {
        let mut blocs = Vec::new();
        let mut current = LocList::default();
        let Some(depth) = self.locs.first()
            .map(|loc| loc.min_depth())
            else { return blocs };
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
}
impl Focused {
    pub fn print_debug(&self) -> () {
        self.before.print_debug(" BEFORE ");
        self.focus.print_debug(" FOCUS ");
        self.after.print_debug(" AFTER ");
    }
    pub fn sort(self) -> LocList {
        let mut locs = self.before.locs;
        let mut blocks = self.focus.into_blocks();
        blocks.sort();
        for block in blocks {
            locs.extend(block.locs);
        }
        locs.extend(self.after.locs);
        LocList { locs }
    }
}
impl PartialEq for Loc {
    fn eq(&self, other: &Self) -> bool {
        self.sort_key == other.sort_key
    }
}
impl Eq for Loc {}
impl Ord for Loc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key.cmp(&other.sort_key)
    }
}
impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for LocList {
    fn eq(&self, other: &Self) -> bool {
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
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ia = 0;
        let mut ib = 0;
        loop {
            while (ia < self.locs.len() && !self.locs[ia].is_sortable()) {
                ia += 1;
            }
            while (ib < other.locs.len() && !other.locs[ib].is_sortable()) {
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
        unreachable!();
    }
}
impl PartialOrd for LocList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn range_around_idx(
    locs: &[Loc],
    line_idx: LineIndex,
) -> CsResult<LineNumberRange> {
    if line_idx >= locs.len() {
        return Err(CsError::NoSortableRangeAround(line_idx));
    }
    let mut start = line_idx;
    let mut end = line_idx;
    while start > 0 && locs[start-1].min_depth() >= locs[line_idx].min_depth() {
        start -= 1;
    }
    while end < locs.len() - 1 && locs[end + 1].min_depth() >= locs[line_idx].min_depth() {
        end += 1;
    }
    Ok(LineNumberRange {
        start: LineNumber::from_index(start),
        end: LineNumber::from_index(end),
    })
}

#[derive(Debug, Clone, Default)]
pub struct BraceStack {
    pub braces: Vec<char>,
}

pub trait Analyzer {

    fn read<R: std::io::BufRead>(
        &self,
        reader: R,
    ) -> CsResult<LocList>;

    fn read_str(
        &self,
        s: &str,
    ) -> CsResult<LocList> {
        self.read(s.as_bytes())
    }

}

pub struct RustAnalyzer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Normal,
    DoubleQuotedString,
    RawString(usize),
    LineComment,
    StarComment,
}

impl BraceStack {
    pub fn push(&mut self, brace: char) -> CsResult<()>{
        match brace {
            '(' | '[' | '{' => self.braces.push(brace),
            ')' => {
                if self.braces.pop() != Some('(') {
                    return Err(CsError::InputNotBalanced);
                }
            }
            ']' => {
                if self.braces.pop() != Some('[') {
                    return Err(CsError::InputNotBalanced);
                }
            }
            '}' => {
                if self.braces.pop() != Some('{') {
                    return Err(CsError::InputNotBalanced);
                }
            }
            _ => panic!("unexpected brace: {}", brace),
        }
        Ok(())
    }
    pub fn depth(&self) -> usize {
        self.braces.len()
    }
}


impl Analyzer for RustAnalyzer {

    // keep track of the byte index of the current line

    fn read<R: std::io::BufRead>(
        &self,
        mut reader: R,
    ) -> CsResult<LocList> {
        let mut locs = Vec::new();
        let mut braces = BraceStack::default();
        let mut last_is_antislash = false;
        let mut state = State::Normal;
        loop {
            if state == State::LineComment {
                state = State::Normal;
            }
            let mut content = String::new();
            let n = reader.read_line(&mut content)?;
            if n == 0 {
                break;
            }
            let start_depth = braces.depth();
            let indented = content.trim_start();
            let bytes = indented.as_bytes();
            let indent = content.len() - indented.len();
            let mut chars = indented.char_indices();
            let mut sort_key = String::new();
            loop {
                let Some((i,c)) = chars.next() else { break };
                match state {
                    State::Normal => {
                        match c {
                            '"' if !last_is_antislash => {
                                // let's count the `#` before and determine whether it's
                                // a raw string or not
                                let mut sharp_count = 0;
                                for j in (1..i).rev() {
                                    if bytes[j] == b'#' {
                                        sharp_count += 1;
                                    } else {
                                        break;
                                    }
                                }
                                state = if sharp_count > 0 && bytes[i - sharp_count - 1] == b'r' {
                                    State::RawString(sharp_count)
                                } else {
                                    State::DoubleQuotedString
                                };
                                sort_key.push(c);
                            }
                            '/' if !last_is_antislash => {
                                if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                                    state = State::LineComment;
                                } else if i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                                    state = State::StarComment;
                                } else {
                                    sort_key.push(c);
                                }
                            }
                            c if char_is_brace(c) && !last_is_antislash => {
                                if i > 1 && bytes[i - 1] == b'\'' && i + 1 < bytes.len() && bytes[i + 1] == b'\'' {
                                    // it's a char literal
                                } else {
                                    braces.push(c);
                                }
                                sort_key.push(c);
                            }
                            ' ' | '\t' | '\n' | '\r' if !last_is_antislash => {
                                // ignore
                            }
                            c => {
                                sort_key.push(c);
                            }
                        }
                        last_is_antislash = c == '\\' && !last_is_antislash;
                    }
                    State::DoubleQuotedString => {
                        match c {
                            '"' if !last_is_antislash => {
                                state = State::Normal;
                            }
                            _ => {}
                        }
                    }
                    State::RawString(sharp_count) => {
                        match c {
                            '#' if i > sharp_count + 1 && bytes[i-sharp_count] == b'"' => {
                                state = State::Normal;
                                for j in 0..sharp_count {
                                    if bytes[i-j] != b'#' {
                                        state = State::RawString(sharp_count);
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    State::LineComment => {
                        // ignore
                    }
                    State::StarComment => {
                        match c {
                            '/' if i > 0 && bytes[i - 1] == b'*' => {
                                state = State::Normal;
                            }
                            _ => {}
                        }
                    }
                }
            }
            let is_annotation = sort_key.starts_with("#[");
            let last_significant_char = sort_key.chars().rev().find(|c| !c.is_whitespace());
            let can_complete = last_significant_char
                .map_or(false, |c| char_is_brace(c) || c == ',' || c == ';');
            locs.push(Loc {
                content,
                sort_key,
                indent,
                start_depth,
                end_depth: braces.depth(),
                braces_at_end: braces.clone(),
                is_annotation,
                can_complete,
            });
        }
        Ok(LocList{locs})
    }

}

fn char_is_brace(c: char) -> bool {
    match c {
        '{' | '}' | '[' | ']' | '(' | ')' => true,
        _ => false,
    }
}
fn braces_are_balanced(braces: &[char]) -> bool {
    let len = braces.len();
    if len % 2 != 0 {
        return false;
    }
    let mut stack = Vec::new();
    for &brace in braces {
        match brace {
            '(' | '[' | '{' => stack.push(brace),
            ')' => match stack.pop() {
                Some('(') => (),
                _ => return false,
            },
            ']' => match stack.pop() {
                Some('[') => (),
                _ => return false,
            },
            '}' => match stack.pop() {
                Some('{') => (),
                _ => return false,
            },
            _ => panic!("unexpected brace: {}", brace),
        }
    }
    stack.is_empty()
}

#[test]
fn test_provisoire() {
    static INPUT: &str = r#"
    impl From<SpecialHandlingShortcut> for SpecialHandling {
        fn from(shortcut: SpecialHandlingShortcut) -> Self {
            use Directive::*;
            match shortcut {
                SpecialHandlingShortcut::None => SpecialHandling {
                    show: Default, list: Default, sum: Default,
                },
                SpecialHandlingShortcut::Enter => SpecialHandling {
                    show: Always, list: Always, sum: Always,
                },
                SpecialHandlingShortcut::NoEnter => SpecialHandling {
                    show: Default, list: Never, sum: Never,
                },
                /// when it's hiding
                #[cfg(feature = "hide")]
                SpecialHandlingShortcut::Hide => SpecialHandling {
                    show: Never, list: Default, sum: Never,
                },
                SpecialHandlingShortcut::NoHide => SpecialHandling {
                    show: Always, list: Default, sum: Default,
                },
            }
        }
    }
    "#;
    static OUTPUT: &str = r#"
    impl From<SpecialHandlingShortcut> for SpecialHandling {
        fn from(shortcut: SpecialHandlingShortcut) -> Self {
            use Directive::*;
            match shortcut {
                SpecialHandlingShortcut::Enter => SpecialHandling {
                    show: Always, list: Always, sum: Always,
                },
                SpecialHandlingShortcut::Hide => SpecialHandling {
                    show: Never, list: Default, sum: Never,
                },
                SpecialHandlingShortcut::NoEnter => SpecialHandling {
                    show: Default, list: Never, sum: Never,
                },
                SpecialHandlingShortcut::NoHide => SpecialHandling {
                    show: Always, list: Default, sum: Default,
                },
                SpecialHandlingShortcut::None => SpecialHandling {
                    show: Default, list: Default, sum: Default,
                },
            }
        }
    }
    "#;
    let analyzer = RustAnalyzer;
    let list = analyzer.read_str(INPUT).unwrap();
    //list.print_debug(" WHOLE ");
    assert!(list.locs[5].starts_with("SpecialHandlingShortcut::None"));
    assert!(list.locs[8].starts_with("SpecialHandlingShortcut::Enter"));
    assert!(list.locs[8] < list.locs[5]);
    let range = range_around_idx(&list.locs, 8).unwrap();
    assert_eq!(range, LineNumberRange {
        start: LineNumber::new(6).unwrap(),
        end: LineNumber::new(22).unwrap(),
    });
    let focused = list.focus(range).unwrap();
    //focused.print_debug();
    {
        let blocks = focused.clone().focus.into_blocks();
        for (i, block) in blocks.iter().enumerate() {
            block.print_debug(&format!(" BLOCK {i}"));
        }
        assert!(blocks[1] < blocks[0]);
        assert!(blocks[3] < blocks[0]);
    }
    let sorted_list = focused.sort();
    sorted_list.print_debug(" SORTED ");

    //let window = list.window_around(7).unwrap();
    //let mut output = String::new();
    //write!(output, "{}", window.sort().unwrap()).unwrap();
    //println!("{}", output);
    //assert_eq!(output, OUTPUT);
    todo!();
}
