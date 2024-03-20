use {
    crate::*,
};

#[derive(Debug, Clone)]
pub struct Loc {
    pub content: String,
    pub sort_key: String,
    /// number of bytes of leading spaces
    pub indent: usize,
    pub depth: usize,
    pub braces_at_end: BraceStack,
}

#[derive(Debug, Clone, Default)]
pub struct BraceStack {
    pub braces: Vec<char>,
}

pub trait Analyzer {

    fn read<R: std::io::BufRead>(
        &self,
        reader: R,
    ) -> CsResult<Vec<Loc>>;

    fn read_str(
        &self,
        s: &str,
    ) -> CsResult<Vec<Loc>> {
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
    ) -> CsResult<Vec<Loc>> {
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
            let depth = braces.depth();
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
            locs.push(Loc {
                content,
                sort_key,
                indent,
                depth,
                braces_at_end: braces.clone(),
            });
        }
        Ok(locs)
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
    let locs = analyzer.read_str(INPUT).unwrap();
    dbg!(&locs);

    //let window = list.window_around(7).unwrap();
    //let mut output = String::new();
    //write!(output, "{}", window.sort().unwrap()).unwrap();
    //println!("{}", output);
    //assert_eq!(output, OUTPUT);
    todo!();
}
