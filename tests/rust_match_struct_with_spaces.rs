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
    let list = LocList::read_str(INPUT, Language::Rust).unwrap();
    //list.print_debug(" WHOLE ");
    let focused = list.focus_around_line_index(26).unwrap();
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
