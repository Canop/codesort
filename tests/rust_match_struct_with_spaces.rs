use {
    code_sort::*,
    std::fmt::Write,
};

static INPUT: &str = r#"
pub fn on_event(
    &mut self,
    w: &mut W,
    timed_event: TimedEvent,
    con: &AppContext,
) -> Result<Command, ProgramError> {
    let cmd = match timed_event {

        // a key event
        TimedEvent {
            key_combination: Some(key),
            ..
        } => {
            self.on_key(timed_event, key, con)
        }

        // an event
        // (with a mouse)
        TimedEvent {
            event: Event::Mouse(MouseEvent { kind, column, row }),
            ..
        } => {
            self.on_mouse(timed_event, kind, column, row)
        }

        // anything else
        _ => Command::None,
    };
    self.input_field.display_on(w)?;
    Ok(cmd)
}
"#;

static OUTPUT: &str = r#"
pub fn on_event(
    &mut self,
    w: &mut W,
    timed_event: TimedEvent,
    con: &AppContext,
) -> Result<Command, ProgramError> {
    let cmd = match timed_event {

        // an event
        // (with a mouse)
        TimedEvent {
            event: Event::Mouse(MouseEvent { kind, column, row }),
            ..
        } => {
            self.on_mouse(timed_event, kind, column, row)
        }

        // a key event
        TimedEvent {
            key_combination: Some(key),
            ..
        } => {
            self.on_key(timed_event, key, con)
        }

        // anything else
        _ => Command::None,
    };
    self.input_field.display_on(w)?;
    Ok(cmd)
}
"#;

#[test]
fn test_match_struct_with_spaces() {
    let list: List = INPUT.parse().unwrap();
    assert!(list.lines[26].content().contains("// anything else"));
    let window = list.window_around(26).unwrap();
    dbg!((window.start, window.end));
    assert_eq!(window.len(), 20);
    let mut output = String::new();
    write!(output, "{}", window.sort().unwrap()).unwrap();
    println!("{}", output);
    assert_eq!(output, OUTPUT);
}
