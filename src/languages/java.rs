use {
    super::*,
    crate::*,
};

/// Return a Balanced if the given code is balanced according to
/// Java syntax.
///
/// This one isn't tested yet, because I don't have enough
/// open-source Java code to test it with.
pub fn check_balanced(s: &str) -> Option<Balanced> {
    let bytes = s.as_bytes();
    let mut sort_key = String::new();
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
                // it's a string
                loop {
                    let Some((_, &c)) = iter.next() else {
                        return None; // unclosed string
                    };
                    if c == b'"' && !last_is_antislash {
                        break; // end of string
                    } else {
                        sort_key.push(c as char);
                    }
                    last_is_antislash = c == b'\\';
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
                        let (_, &c) = iter.next()?;
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
                sort_key.push(c as char);
            }
            b' ' | b'\t' | b'\n' | b'\r' if !last_is_antislash => {
                // ignore
            }
            b'\\' if !last_is_antislash => {
                last_significant_char = Some(c);
                last_is_antislash = true;
            }
            c => {
                sort_key.push(c as char);
                last_significant_char = Some(c);
                last_is_antislash = false;
            }
        }
    }
    let last_significant_char = last_significant_char.map(|c| c as char);
    if !braces_are_balanced(&braces) {
        return None;
    }
    Some(Balanced {
        is_annotation: sort_key.starts_with('@'),
        sort_key,
        last_significant_char,
        language: Language::Rust,
    })
}
