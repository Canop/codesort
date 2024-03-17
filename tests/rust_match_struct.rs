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
        TimedEvent { // a tricky comment
            event: Event::Mouse(MouseEvent { kind, column, row }),
            ..
        } => { // inner block
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
        TimedEvent { // a tricky comment
            event: Event::Mouse(MouseEvent { kind, column, row }),
            ..
        } => { // inner block
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
fn test_match_struct() {
    let list: List = INPUT.parse().unwrap();
    println!("{}", list.lines[17].content());
    assert!(list.lines[17].starts_with("TimedEvent {"));
    let window = list.window_around(17).unwrap();
    dbg!((window.start, window.end));
    assert_eq!(window.len(), 17);
    let mut output = String::new();
    write!(output, "{}", window.sort()).unwrap();
    assert_eq!(output, OUTPUT);
}
