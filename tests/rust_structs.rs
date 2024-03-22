use codesort::*;

static INPUT: &str = r#"
/// short lived wrapping of a few things which are needed for the handling
/// of a command in a panel and won't be modified during the operation.
pub struct CmdContext<'c> {
    pub cmd: &'c Command,
    pub app: &'c AppCmdContext<'c>,
    pub panel: PanelCmdContext<'c>,
}

/// the part of the immutable command execution context which comes from the app
pub struct AppCmdContext<'c> {
    pub panel_skin: &'c PanelSkin,
    pub preview_panel: Option<PanelId>, // id of the app's preview panel
    pub stage_panel: Option<PanelId>, // id of the app's preview panel
    pub screen: Screen,
    pub con: &'c AppContext,
}

/// the part of the command execution context which comes from the panel
pub struct PanelCmdContext<'c> {

    pub areas: &'c Areas,

    pub purpose: PanelPurpose,

}
"#;

static OUTPUT: &str = r#"
/// the part of the immutable command execution context which comes from the app
pub struct AppCmdContext<'c> {
    pub panel_skin: &'c PanelSkin,
    pub preview_panel: Option<PanelId>, // id of the app's preview panel
    pub stage_panel: Option<PanelId>, // id of the app's preview panel
    pub screen: Screen,
    pub con: &'c AppContext,
}

/// short lived wrapping of a few things which are needed for the handling
/// of a command in a panel and won't be modified during the operation.
pub struct CmdContext<'c> {
    pub cmd: &'c Command,
    pub app: &'c AppCmdContext<'c>,
    pub panel: PanelCmdContext<'c>,
}

/// the part of the command execution context which comes from the panel
pub struct PanelCmdContext<'c> {

    pub areas: &'c Areas,

    pub purpose: PanelPurpose,

}
"#;

#[test]
fn test_match_struct() {
    let list = LocList::read_str(INPUT, Language::Rust).unwrap();
    //list.print_debug(" WHOLE ");
    let focused = list.focus_all().unwrap();
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
