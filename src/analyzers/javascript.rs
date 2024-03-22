use crate::*;

pub struct JavascriptAnalyzer;

/// A state which goes beyond line boundaries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Normal,
    DoubleQuotedString,
    SingleQuotedString,
    LineComment,
    StarComment,
}

impl Analyzer for JavascriptAnalyzer {
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
            let wishes = Vec::new(); // not used in java
            let gifts = Vec::new(); // not used in java
            loop {
                let Some((i, c)) = chars.next() else { break };
                match state {
                    State::Normal => {
                        match c {
                            '\'' if !last_is_antislash => {
                                state = State::SingleQuotedString;
                                sort_key.push(c);
                            }
                            '"' if !last_is_antislash => {
                                state = State::DoubleQuotedString;
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
                                braces.push(c)?; // error if unbalanced
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
                    State::SingleQuotedString => {
                        if c == '\'' && !last_is_antislash {
                            state = State::Normal;
                        }
                        sort_key.push(c);
                    }
                    State::DoubleQuotedString => {
                        if c == '"' && !last_is_antislash {
                            state = State::Normal;
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
            let can_complete =
                last_significant_char.map_or(false, |c| char_is_brace(c) || c == ';');
            let loc = Loc {
                content,
                sort_key,
                indent,
                start_depth,
                end_depth: braces.depth(),
                is_annotation,
                can_complete,
                wishes,
                gifts,
            };
            locs.push(loc);
        }
        Ok(LocList { locs })
    }
}

#[test]
fn test_balance_javascript() {
    let code = r#"
        // called by the server or (most often) in case of any action on a message
        //  (so this is very frequently called on non pings)
        notif.removePing = function(mid, forwardToServer, flash){
            if (!mid) return;
            // we assume here there's at most one notification to a given message
            for (var i=0; i<notifications.length; i++) {
                if (notifications[i].mid==mid) {
                    if (flash) {
                        var $md = $('#messages .message[mid='+mid+']');
                        if ($md.length) {
                            md.goToMessageDiv($md);
                        }
                    }
                    if (forwardToServer) ws.emit("rm_ping", mid);
                    notifications.splice(i, 1);
                    notif.updatePingsList();
                    return;
                }
            }
        }
    "#;
    let analyzer = JavascriptAnalyzer;
    let list = analyzer.read_str(code).unwrap();
    assert!(list.is_complete());
}
