use {
    code_sort::*,
    std::fmt::Write,
};

static INPUT: &str = r#"
    let con = &cc.app.con;
    let screen = cc.app.screen;
    let bang = input_invocation
        .map(|inv| inv.bang)
        .unwrap_or(internal_exec.bang);
    Ok(match internal_exec.internal {
        Internal::back => CmdResult::PopState,
        Internal::close_panel_ok => CmdResult::ClosePanel {
            validate_purpose: true,
            panel_ref: PanelReference::Active,
        },
        Internal::close_panel_cancel => CmdResult::ClosePanel {
            validate_purpose: false,
            panel_ref: PanelReference::Active,
        },
        #[cfg(unix)]
        Internal::filesystems => {

            let fs_state = crate::filesystems::FilesystemState::new(
                self.selected_path(),
                self.tree_options(),
                con,
            );

            match fs_state {
                Ok(state) => {
                    let bang = input_invocation
                        .map(|inv| inv.bang)
                        .unwrap_or(internal_exec.bang);
                    if bang && cc.app.preview_panel.is_none() {
                        CmdResult::NewPanel {
                            state: Box::new(state),
                            purpose: PanelPurpose::None,
                            direction: HDir::Right,
                        }
                    } else {
                        CmdResult::new_state(Box::new(state))
                    }
                }
                Err(e) => CmdResult::DisplayError(format!("{e}")),
            }

        }
        Internal::mode_input => self.on_mode_verb(Mode::Input, con),
        Internal::mode_command => self.on_mode_verb(Mode::Command, con),
        Internal::open_leave => {
            if let Some(selection) = self.selection() {
                selection.to_opener(con)?
            } else {
                CmdResult::error("no selection to open")
            }
        }
        Internal::open_preview => self.open_preview(None, false, cc),
        Internal::preview_image => self.open_preview(Some(PreviewMode::Image), false, cc),
        Internal::preview_text => self.open_preview(Some(PreviewMode::Text), false, cc),
        Internal::preview_binary => self.open_preview(Some(PreviewMode::Hex), false, cc),
        Internal::escape => {
            CmdResult::HandleInApp(Internal::escape)
        }
        Internal::clear_stage => {
            app_state.stage.clear();
            if let Some(panel_id) = cc.app.stage_panel {
                CmdResult::ClosePanel {
                    validate_purpose: false,
                    panel_ref: PanelReference::Id(panel_id),
                }
            } else {
                CmdResult::Keep
            }
        }
        Internal::stage => self.stage(app_state, cc, con),
        Internal::unstage => self.unstage(app_state, cc, con),
        Internal::toggle_stage => self.toggle_stage(app_state, cc, con),
        _ => CmdResult::Keep,
    })
"#;

static OUTPUT: &str = r#"
    let con = &cc.app.con;
    let screen = cc.app.screen;
    let bang = input_invocation
        .map(|inv| inv.bang)
        .unwrap_or(internal_exec.bang);
    Ok(match internal_exec.internal {
        Internal::back => CmdResult::PopState,
        Internal::clear_stage => {
            app_state.stage.clear();
            if let Some(panel_id) = cc.app.stage_panel {
                CmdResult::ClosePanel {
                    validate_purpose: false,
                    panel_ref: PanelReference::Id(panel_id),
                }
            } else {
                CmdResult::Keep
            }
        }
        Internal::close_panel_cancel => CmdResult::ClosePanel {
            validate_purpose: false,
            panel_ref: PanelReference::Active,
        },
        Internal::close_panel_ok => CmdResult::ClosePanel {
            validate_purpose: true,
            panel_ref: PanelReference::Active,
        },
        Internal::escape => {
            CmdResult::HandleInApp(Internal::escape)
        }
        #[cfg(unix)]
        Internal::filesystems => {

            let fs_state = crate::filesystems::FilesystemState::new(
                self.selected_path(),
                self.tree_options(),
                con,
            );

            match fs_state {
                Ok(state) => {
                    let bang = input_invocation
                        .map(|inv| inv.bang)
                        .unwrap_or(internal_exec.bang);
                    if bang && cc.app.preview_panel.is_none() {
                        CmdResult::NewPanel {
                            state: Box::new(state),
                            purpose: PanelPurpose::None,
                            direction: HDir::Right,
                        }
                    } else {
                        CmdResult::new_state(Box::new(state))
                    }
                }
                Err(e) => CmdResult::DisplayError(format!("{e}")),
            }

        }
        Internal::mode_command => self.on_mode_verb(Mode::Command, con),
        Internal::mode_input => self.on_mode_verb(Mode::Input, con),
        Internal::open_leave => {
            if let Some(selection) = self.selection() {
                selection.to_opener(con)?
            } else {
                CmdResult::error("no selection to open")
            }
        }
        Internal::open_preview => self.open_preview(None, false, cc),
        Internal::preview_binary => self.open_preview(Some(PreviewMode::Hex), false, cc),
        Internal::preview_image => self.open_preview(Some(PreviewMode::Image), false, cc),
        Internal::preview_text => self.open_preview(Some(PreviewMode::Text), false, cc),
        Internal::stage => self.stage(app_state, cc, con),
        Internal::toggle_stage => self.toggle_stage(app_state, cc, con),
        Internal::unstage => self.unstage(app_state, cc, con),
        _ => CmdResult::Keep,
    })
"#;

#[test]
fn test_match_big_enum() {
    let list: List = INPUT.parse().unwrap();
    println!("{}", list.lines[7]);
    assert!(list.lines[7].starts_with("Internal"));
    let window = list.window_around(7);
    let mut output = String::new();
    write!(output, "{}", window.sort()).unwrap();
    println!("{}", output);
    assert_eq!(output, OUTPUT);
}
