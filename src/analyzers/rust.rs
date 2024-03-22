use crate::*;

pub struct RustAnalyzer;

/// A state which goes beyond line boundaries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Normal,
    DoubleQuotedString,
    RawString(usize),
    LineComment,
    StarComment,
}

fn char_is_gift(c: char) -> bool {
    match c {
        '(' | '{' | ';' => true,
        _ => false,
    }
}

/// Return what the token calls for
fn token_wishes(token: &str) -> Vec<CharSet> {
    match token {
        "fn" => {
            vec!['('.into(), vec!['{', ';'].into()]
        }
        "impl" | "enum" | "trait" | "mod" | "match" => {
            vec!['{'.into()]
        }
        _ => vec![],
    }
}

impl Analyzer for RustAnalyzer {
    fn read<R: std::io::BufRead>(
        &self,
        mut reader: R,
    ) -> CsResult<LocList> {
        let mut locs = Vec::new();
        let mut braces = BraceStack::default();
        let mut last_is_antislash = false;
        let mut last_is_quote = false;
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
            let mut current_token = String::new();
            let mut wishes = Vec::new();
            let mut gifts = Vec::new();
            loop {
                let Some((i, c)) = chars.next() else { break };
                let token = if c.is_ascii_alphabetic() {
                    // we're only interested in possible keywords
                    current_token.push(c);
                    None
                } else {
                    Some(std::mem::take(&mut current_token))
                };
                match state {
                    State::Normal => {
                        if let Some(token) = token.as_ref() {
                            for any_of in token_wishes(token) {
                                wishes.push(Wish {
                                    depth: braces.depth(),
                                    any_of,
                                });
                            }
                        }
                        if char_is_gift(c) {
                            let gift = Gift {
                                depth: braces.depth(),
                                c,
                            };
                            if let Some(bix) =
                                wishes.iter().rposition(|wish| gift.satisfies(wish))
                            {
                                wishes.remove(bix);
                            } else {
                                gifts.push(gift);
                            }
                        }
                        match c {
                            '"' if !last_is_antislash && !last_is_quote => {
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
                                state = if sharp_count > 0
                                    && bytes[i - sharp_count - 1] == b'r'
                                {
                                    State::RawString(sharp_count)
                                } else {
                                    State::DoubleQuotedString
                                };
                                sort_key.push(c);
                            }
                            '/' if !last_is_antislash && !last_is_quote => {
                                if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                                    state = State::LineComment;
                                } else if i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                                    state = State::StarComment;
                                } else {
                                    sort_key.push(c);
                                }
                            }
                            c if char_is_brace(c)
                                && !last_is_antislash
                                && !last_is_quote =>
                            {
                                braces.push(c)?; // error if unbalanced
                                sort_key.push(c);
                            }
                            ' ' | '\t' | '\n' | '\r'
                                if !last_is_antislash && !last_is_quote =>
                            {
                                // ignore
                            }
                            c => {
                                sort_key.push(c);
                            }
                        }
                        last_is_antislash = c == '\\' && !last_is_antislash;
                        last_is_quote = c == '\'';
                    }
                    State::DoubleQuotedString => {
                        if c == '"' && !last_is_antislash {
                            state = State::Normal;
                        }
                        sort_key.push(c);
                    }
                    State::RawString(sharp_count) => {
                        if c == '#'
                            && i > sharp_count + 1
                            && bytes[i - sharp_count] == b'"'
                        {
                            state = State::Normal;
                            for j in 0..sharp_count {
                                if bytes[i - j] != b'#' {
                                    state = State::RawString(sharp_count);
                                    break;
                                }
                            }
                        }
                        sort_key.push(c);
                    }
                    State::LineComment => {
                        // ignore
                    }
                    State::StarComment => match c {
                        '/' if i > 0 && bytes[i - 1] == b'*' => {
                            state = State::Normal;
                        }
                        _ => {}
                    },
                }
            }
            let is_annotation = sort_key.starts_with("#[");
            let last_significant_char =
                sort_key.chars().rev().find(|c| !c.is_whitespace());
            let can_complete = last_significant_char
                .map_or(false, |c| char_is_brace(c) || c == ',' || c == ';');
            locs.push(Loc {
                content,
                sort_key,
                indent,
                start_depth,
                end_depth: braces.depth(),
                is_annotation,
                can_complete,
                wishes,
                gifts,
            });
        }
        Ok(LocList { locs })
    }
}

#[test]
fn test_match_arms() {
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
                /// when it's hiding
                #[cfg(feature = "hide")]
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
    list.print_debug(" WHOLE ");
    assert!(list.locs[5].starts_with("SpecialHandlingShortcut::None"));
    assert!(list.locs[8].starts_with("SpecialHandlingShortcut::Enter"));
    assert!(list.locs[8] < list.locs[5]);
    let range = list.range_around_idx(8).unwrap();
    assert_eq!(
        range,
        LineNumberRange {
            start: LineNumber::new(6).unwrap(),
            end: LineNumber::new(22).unwrap(),
        }
    );
    let focused = list.focus(range).unwrap();
    focused.print_debug();
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
    assert_eq!(sorted_list.to_string(), OUTPUT);
}

#[test]
fn test_where_comma() {
    static INPUT: &str = r#"
        trait Foo {}

        struct Bar<T>(T);
        struct Baz<T>(T);
        struct Blee<T>(T);

        impl<T> Foo for Blee<T>
        where
            T: Copy,
        {
            // ...
        }

        impl<T> Foo for Baz<T>
        where
            T: Copy,
        {
            // ...
        }

        impl<T> Foo for Bar<T>
        where
            T: Copy,
        {
            // ...
        }

        fn main() {}
    "#;
    static OUTPUT: &str = r#"

        fn main() {}

        impl<T> Foo for Bar<T>
        where
            T: Copy,
        {
            // ...
        }

        impl<T> Foo for Baz<T>
        where
            T: Copy,
        {
            // ...
        }

        impl<T> Foo for Blee<T>
        where
            T: Copy,
        {
            // ...
        }

        struct Bar<T>(T);
        struct Baz<T>(T);
        struct Blee<T>(T);

        trait Foo {}
    "#;
    let analyzer = RustAnalyzer;
    let list = analyzer.read_str(INPUT).unwrap();
    //list.print_debug(" WHOLE ");
    let focused = list.focus_all().unwrap();
    focused.print_debug();
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
    assert_eq!(sorted_list.to_string().trim(), OUTPUT.trim());
}

/// Traps:
/// - badly indented code (because it's not easy with raw strings)
/// - quoted double-quote
#[test]
fn test_check_balanced_rust() {
    let test_cases = vec![
        r#"let is_double_quote = if c == b'"' {
    true
        } else {
            false
        };  // the end"#,
        r#"   SpecialHandlingShortcut::None => SpecialHandling {
           show: Default, list: Default, sum: Default,
       },"#,
    ];
    for code in test_cases {
        let analyzer = RustAnalyzer;
        let list = analyzer.read_str(code).unwrap();
        assert!(list.is_complete());
    }
}
