use codesort::*;

static INPUT: &str = r#"
/// a panel state, stackable to allow reverting
///  to a previous one
pub trait PanelState {

    fn get_type(&self) -> PanelStateType;

    fn set_mode(&mut self, mode: Mode);

    fn get_mode(&self) -> Mode;

    /// called on start of on_command
    fn clear_pending(&mut self) {}

    fn on_click(
        &mut self,
        _x: u16,
        _y: u16,
        _screen: Screen,
        _con: &AppContext,
    ) -> Result<CmdResult, ProgramError> {
        Ok(CmdResult::Keep)
    }

    fn on_double_click(
        &mut self,
        _x: u16,
        _y: u16,
        _screen: Screen,
        _con: &AppContext,
    ) -> Result<CmdResult, ProgramError> {
        Ok(CmdResult::Keep)
    }

    fn on_pattern(
        &mut self,
        _pat: InputPattern,
        _app_state: &AppState,
        _con: &AppContext,
    ) -> Result<CmdResult, ProgramError> {
        Ok(CmdResult::Keep)
    }

    /// execute the internal with the optional given invocation.
    ///
    /// The invocation comes from the input and may be related
    /// to a different verb (the verb may have been triggered
    /// by a key shortcut)
    #[allow(clippy::too_many_arguments)]
    fn on_internal(
        &mut self,
        w: &mut W,
        invocation_parser: Option<&InvocationParser>,
        internal_exec: &InternalExecution,
        input_invocation: Option<&VerbInvocation>,
        trigger_type: TriggerType,
        app_state: &mut AppState,
        cc: &CmdContext,
    ) -> Result<CmdResult, ProgramError>;

    fn execute_sequence(
        &mut self,
        _w: &mut W,
        verb: &Verb,
        seq_ex: &SequenceExecution,
        invocation: Option<&VerbInvocation>,
        app_state: &mut AppState,
        cc: &CmdContext,
    ) -> Result<CmdResult, ProgramError> {
        let sel_info = self.sel_info(app_state);
        if matches!(sel_info, SelInfo::More(_)) {
            // sequences would be hard to execute as the execution on a file can change the
            // state in too many ways (changing selection, focused panel, parent, unstage or
            // stage files, removing the staged paths, etc.)
            return Ok(CmdResult::error("sequences can't be executed on multiple selections"));
        }

        let exec_builder = ExecutionStringBuilder::with_invocation(
            verb.invocation_parser.as_ref(),
            sel_info,
            app_state,
            if let Some(inv) = invocation {
                inv.args.as_ref()
            } else {
                None
            },
        );
        let sequence = exec_builder.sequence(&seq_ex.sequence, &cc.app.con.verb_store);
        Ok(CmdResult::ExecuteSequence { sequence })
    }

    fn has_at_least_one_selection(&self, _app_state: &AppState) -> bool {
        true // overloaded in stage_state
    }

    fn refresh(&mut self, screen: Screen, con: &AppContext) -> Command;

    fn tree_options(&self) -> TreeOptions;

    /// Build a cmdResult in response to a command being a change of
    /// tree options. This may or not be a new state.
    ///
    /// The provided `change_options` function returns a status message
    /// explaining the change
    fn with_new_options(
        &mut self,
        screen: Screen,
        change_options: &dyn Fn(&mut TreeOptions) -> &'static str,
        in_new_panel: bool,
        con: &AppContext,
    ) -> CmdResult;

}
"#;

static OUTPUT: &str = r#"
/// a panel state, stackable to allow reverting
///  to a previous one
pub trait PanelState {

    /// called on start of on_command
    fn clear_pending(&mut self) {}

    fn execute_sequence(
        &mut self,
        _w: &mut W,
        verb: &Verb,
        seq_ex: &SequenceExecution,
        invocation: Option<&VerbInvocation>,
        app_state: &mut AppState,
        cc: &CmdContext,
    ) -> Result<CmdResult, ProgramError> {
        let sel_info = self.sel_info(app_state);
        if matches!(sel_info, SelInfo::More(_)) {
            // sequences would be hard to execute as the execution on a file can change the
            // state in too many ways (changing selection, focused panel, parent, unstage or
            // stage files, removing the staged paths, etc.)
            return Ok(CmdResult::error("sequences can't be executed on multiple selections"));
        }

        let exec_builder = ExecutionStringBuilder::with_invocation(
            verb.invocation_parser.as_ref(),
            sel_info,
            app_state,
            if let Some(inv) = invocation {
                inv.args.as_ref()
            } else {
                None
            },
        );
        let sequence = exec_builder.sequence(&seq_ex.sequence, &cc.app.con.verb_store);
        Ok(CmdResult::ExecuteSequence { sequence })
    }

    fn get_mode(&self) -> Mode;

    fn get_type(&self) -> PanelStateType;

    fn has_at_least_one_selection(&self, _app_state: &AppState) -> bool {
        true // overloaded in stage_state
    }

    fn on_click(
        &mut self,
        _x: u16,
        _y: u16,
        _screen: Screen,
        _con: &AppContext,
    ) -> Result<CmdResult, ProgramError> {
        Ok(CmdResult::Keep)
    }

    fn on_double_click(
        &mut self,
        _x: u16,
        _y: u16,
        _screen: Screen,
        _con: &AppContext,
    ) -> Result<CmdResult, ProgramError> {
        Ok(CmdResult::Keep)
    }

    /// execute the internal with the optional given invocation.
    ///
    /// The invocation comes from the input and may be related
    /// to a different verb (the verb may have been triggered
    /// by a key shortcut)
    #[allow(clippy::too_many_arguments)]
    fn on_internal(
        &mut self,
        w: &mut W,
        invocation_parser: Option<&InvocationParser>,
        internal_exec: &InternalExecution,
        input_invocation: Option<&VerbInvocation>,
        trigger_type: TriggerType,
        app_state: &mut AppState,
        cc: &CmdContext,
    ) -> Result<CmdResult, ProgramError>;

    fn on_pattern(
        &mut self,
        _pat: InputPattern,
        _app_state: &AppState,
        _con: &AppContext,
    ) -> Result<CmdResult, ProgramError> {
        Ok(CmdResult::Keep)
    }

    fn refresh(&mut self, screen: Screen, con: &AppContext) -> Command;

    fn set_mode(&mut self, mode: Mode);

    fn tree_options(&self) -> TreeOptions;

    /// Build a cmdResult in response to a command being a change of
    /// tree options. This may or not be a new state.
    ///
    /// The provided `change_options` function returns a status message
    /// explaining the change
    fn with_new_options(
        &mut self,
        screen: Screen,
        change_options: &dyn Fn(&mut TreeOptions) -> &'static str,
        in_new_panel: bool,
        con: &AppContext,
    ) -> CmdResult;

}
"#;

#[test]
fn test_trait_functions() {
    let list = LocList::read_str(INPUT, Language::Rust).unwrap();
    let focused = list.focus_around_line_index(12).unwrap();
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
