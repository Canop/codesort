use {
    code_sort::*,
    std::fmt::Write,
};

static INPUT: &str = r#"
/// The configuration read from conf.toml or conf.hjson file(s)
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Conf {

    #[serde(alias="default-flags")]
    pub default_flags: Option<String>, // the flags to apply before cli ones

    #[serde(alias="date-time-format")]
    pub date_time_format: Option<String>,

    #[serde(default)]
    pub verbs: Vec<VerbConf>,

    pub skin: Option<AHashMap<String, SkinEntry>>,

    #[serde(default, alias="special-paths")]
    pub special_paths: HashMap<GlobConf, SpecialHandlingConf>,

    #[serde(alias="search-modes")]
    pub search_modes: Option<FnvHashMap<String, String>>,

    /// Obsolete, kept for compatibility: you should now use capture_mouse
    #[serde(alias="disable-mouse-capture")]
    pub disable_mouse_capture: Option<bool>,

    #[serde(alias="cols-order")]
    pub cols_order: Option<ColsConf>,

    #[serde(alias="show-selection-mark")]
    pub show_selection_mark: Option<bool>,

    #[serde(alias="syntax-theme")]
    pub syntax_theme: Option<SyntaxTheme>,

    #[serde(alias="icon-theme")]
    pub icon_theme: Option<String>,

    pub modal: Option<bool>,

    /// the initial mode (only relevant when modal is true)
    #[serde(alias="initial-mode")]
    pub initial_mode: Option<Mode>,

    pub max_panels_count: Option<usize>,

    #[serde(alias="quit-on-last-cancel")]
    pub quit_on_last_cancel: Option<bool>,

    pub file_sum_threads_count: Option<usize>,

    #[serde(alias="max_staged_count")]
    pub max_staged_count: Option<usize>,

    #[serde(default)]
    pub imports: Vec<Import>,

    #[serde(alias="terminal-title")]
    pub terminal_title: Option<ExecPattern>,

    #[serde(alias="update-work-dir")]
    pub update_work_dir: Option<bool>,

    #[serde(alias="kitty-graphics-transmission")]
    pub kitty_graphics_transmission: Option<TransmissionMedium>,
}
"#;

static OUTPUT: &str = r#"
/// The configuration read from conf.toml or conf.hjson file(s)
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Conf {

    #[serde(alias="cols-order")]
    pub cols_order: Option<ColsConf>,

    #[serde(alias="date-time-format")]
    pub date_time_format: Option<String>,

    #[serde(alias="default-flags")]
    pub default_flags: Option<String>, // the flags to apply before cli ones

    /// Obsolete, kept for compatibility: you should now use capture_mouse
    #[serde(alias="disable-mouse-capture")]
    pub disable_mouse_capture: Option<bool>,

    pub file_sum_threads_count: Option<usize>,

    #[serde(alias="icon-theme")]
    pub icon_theme: Option<String>,

    #[serde(default)]
    pub imports: Vec<Import>,

    /// the initial mode (only relevant when modal is true)
    #[serde(alias="initial-mode")]
    pub initial_mode: Option<Mode>,

    #[serde(alias="kitty-graphics-transmission")]
    pub kitty_graphics_transmission: Option<TransmissionMedium>,

    pub max_panels_count: Option<usize>,

    #[serde(alias="max_staged_count")]
    pub max_staged_count: Option<usize>,

    pub modal: Option<bool>,

    #[serde(alias="quit-on-last-cancel")]
    pub quit_on_last_cancel: Option<bool>,

    #[serde(alias="search-modes")]
    pub search_modes: Option<FnvHashMap<String, String>>,

    #[serde(alias="show-selection-mark")]
    pub show_selection_mark: Option<bool>,

    pub skin: Option<AHashMap<String, SkinEntry>>,

    #[serde(default, alias="special-paths")]
    pub special_paths: HashMap<GlobConf, SpecialHandlingConf>,

    #[serde(alias="syntax-theme")]
    pub syntax_theme: Option<SyntaxTheme>,

    #[serde(alias="terminal-title")]
    pub terminal_title: Option<ExecPattern>,

    #[serde(alias="update-work-dir")]
    pub update_work_dir: Option<bool>,

    #[serde(default)]
    pub verbs: Vec<VerbConf>,
}
"#;

#[test]
fn test_struct_fields() {
    let list = List::from_str(INPUT, Language::Rust).unwrap();
    let window = list.window_around(5).unwrap();
    let mut output = String::new();
    write!(output, "{}", window.sort().unwrap()).unwrap();
    assert_eq!(output, OUTPUT);
}
