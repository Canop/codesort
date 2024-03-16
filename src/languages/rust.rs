use {
    super::*,
    crate::*,
};

/// Return a Balanced if the given code is balanced according to
/// Rust syntax.
///
/// rust attributes (i.e. `#[...]`) are not tested because they
/// should be balanced anyway
pub fn check_balanced_rust(s: &str) -> Option<Balanced> {
    let bytes = s.as_bytes();
    let mut braces = Vec::new();
    let mut last_is_antislash = false;
    let mut iter = bytes.iter().enumerate();
    let mut last_significant_char = None;
    loop {
        let Some((i, &c)) = iter.next() else {
            break;
        };
        match c {
            b'"' if !last_is_antislash => {
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
                if sharp_count > 0 && bytes[i - sharp_count - 1] == b'r' {
                    // it's a raw string, let's continue until we find the end
                    loop {
                        let Some((i, &c)) = iter.next() else {
                            return None; // unclosed raw string
                        };
                        if c == b'"' {
                            if (i + 1..i + 1 + sharp_count)
                                .all(|j| j < s.len() && bytes[j] == b'#')
                            {
                                break; // end of raw string
                            }
                        }
                    }
                } else {
                    // it's a normal string
                    loop {
                        let Some((_, &c)) = iter.next() else {
                            return None; // unclosed string
                        };
                        if c == b'"' && !last_is_antislash {
                            break; // end of string
                        }
                        last_is_antislash = c == b'\\';
                    }
                }
            }
            b'/' if !last_is_antislash => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                    // it's a line comment
                    loop {
                        let Some((_, &c)) = iter.next() else {
                            break;
                        };
                        if c == b'\n' {
                            break;
                        }
                    }
                } else if i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                    // it's a block comment
                    loop {
                        let Some((_, &c)) = iter.next() else {
                            return None;
                        };
                        if c == b'*' {
                            if let Some((_, &c)) = iter.next() {
                                if c == b'/' {
                                    break;
                                }
                            } else {
                                return None; // unclosed block comment
                            }
                        }
                    }
                }
            }
            c if char_is_brace(c) && !last_is_antislash => {
                if bytes[i - 1] == b'\'' && i + 1 < bytes.len() && bytes[i + 1] == b'\'' {
                    // it's a char literal
                } else {
                    braces.push(c);
                }
                last_significant_char = Some(c);
            }
            b' ' | b'\t' | b'\n' | b'\r' if !last_is_antislash => {
                // ignore
            }
            b'\\' if !last_is_antislash => {
                last_significant_char = Some(c);
                last_is_antislash = true;
            }
            c => {
                last_significant_char = Some(c);
                last_is_antislash = false;
            }
        }
    }
    let last_significant_char = last_significant_char? as char;
    tprintln!(
        "braces: {}",
        braces.iter().map(|&c| c as char).collect::<String>()
    );
    if !braces_are_balanced(&braces) {
        return None;
    }
    Some(Balanced {
        last_significant_char,
        language: Language::Rust,
    })
}

/// The test cases here should not be balanced until the last line
#[test]
fn test_check_balanced_rust_not_balanced_until_end() {
    let mut test_cases = vec![
        vec![
            "   SpecialHandlingShortcut::None => SpecialHandling {\n",
            "       show: Default, list: Default, sum: Default,\n",
            "   },\n",
        ],
        vec![
            "/// useless comments)\n",
            "match lazy_regex!(r#\"bad regex)\"#) {\n",
            "    Ok(_) => \"ok\",\n",
            "    Err(_) => \"err\",\n",
            "},\n",
        ],
        vec![
            r#"   Internal::open_leave => {\n"#,
            r#"       if let Some(selection) = self.selection() {\n"#,
            r#"           selection.to_opener(con)?\n"#,
            r#"       } else {\n"#,
            r#"           CmdResult::error("no selection to open")\n"#,
            r#"       }\n"#,
            r#"   }\n"#,
        ],
    ];
    for mut lines in test_cases.drain(..) {
        for (i, line) in lines.iter().enumerate() {
            print!("{:>2} | {}", i, line);
        }
        let mut code = String::new();
        let last_line = lines.pop().unwrap();
        for (i, line) in lines.iter().enumerate() {
            code.push_str(line);
            dbg!(check_balanced_rust(&code));
            assert!(
                check_balanced_rust(&code).is_none(),
                "line {} shouldn't balance",
                i
            );
        }
        code.push_str(last_line);
        assert!(
            check_balanced_rust(&code).is_some(),
            "last line should balance"
        );
    }
}

#[test]
fn test_check_balanced_rust_ending_in_comma() {
    let mut test_cases = vec![
        r#"
            SpecialHandlingShortcut::None => SpecialHandling {
                show: Default, list: Default, sum: Default,
            }, // some inane comment
            "#,
        r#"
            /// Obsolete, kept for compatibility: you should now use capture_mouse
            #[serde(alias="disable-mouse-capture")]
            pub disable_mouse_capture: Option<bool>,
            "#,
    ];
    for code in test_cases.drain(..) {
        println!("{}", code);
        let balanced = check_balanced_rust(&code).unwrap();
        assert_eq!(balanced.last_significant_char, ',');
    }
}
