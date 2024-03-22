use codesort::*;

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
    let analyzer = RustAnalyzer;
    let list = analyzer.read_str(INPUT).unwrap();
    let focused = list.focus_around_line_idx(17).unwrap();
    focused.print_debug();
    {
        let blocks = focused.clone().focus.into_blocks();
        for (i, block) in blocks.iter().enumerate() {
            block.print_debug(&format!(" BLOCK {i}"));
        }
    }
    let sorted_list = focused.sort();
    sorted_list.print_debug(" SORTED ");
    assert_eq!(sorted_list.to_string(), OUTPUT);
}
