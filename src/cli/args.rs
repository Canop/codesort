use {
    clap::{
        CommandFactory,
        Parser,
        ValueEnum,
    },
    codesort::*,
    std::path::PathBuf,
    termimad::ansi,
};

static INTRO: &str = "
**codesort** sorts code.

Source & documentation at https://github.com/Canop/codesort
";

/// Launch arguments
#[derive(Debug, Parser)]
#[command(
    author,
    about,
    version,
    disable_version_flag = true,
    disable_help_flag = true
)]
pub struct Args {
    /// Print help information
    #[arg(long)]
    pub help: bool,

    /// Print the version
    #[arg(long)]
    pub version: bool,

    /// Sort the block around this 1-based line number
    #[arg(long, value_name = "LINE")]
    pub around: Option<LineNumber>,

    /// Sort this `start:end` range of 1 based lines, both ends included
    #[arg(long)]
    pub range: Option<LineNumberRange>,

    /// Code language
    #[arg(short, long, default_value = "auto")]
    pub lang: LangChoice,

    /// Path to a file to sort (if not provided, will read from stdin)
    #[arg(long)]
    pub src: Option<PathBuf>,

    /// Where to write after sort (if not provided, will write to stdout)
    #[arg(long)]
    pub dst: Option<PathBuf>,

    /// A path or filename to use only for language detection
    /// (useful when the content is given with stdin)
    #[arg(long, value_name = "PATH")]
    pub detect: Option<PathBuf>,

    /// File to sort in place (shortcut for --src and --dst)
    pub file: Option<PathBuf>,
}

impl Args {
    pub fn print_help(&self) {
        let mut printer = clap_help::Printer::new(Args::command())
            .with("introduction", INTRO)
            .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
            .without("author");
        let skin = printer.skin_mut();
        skin.headers[0].compound_style.set_fg(ansi(79));
        skin.bold.set_fg(ansi(79));
        skin.italic = termimad::CompoundStyle::with_fg(ansi(79));
        printer.print_help();
    }
    pub fn lang(&self) -> Language {
        match self.lang {
            LangChoice::Rust => Language::Rust,
            LangChoice::Java => Language::Java,
            LangChoice::Js => Language::JavaScript,
            LangChoice::Auto => {
                let path = self
                    .detect
                    .as_ref()
                    .or(self.src.as_ref())
                    .or(self.file.as_ref())
                    .or(self.dst.as_ref());
                path.and_then(|p| Language::detect(p))
                    .unwrap_or(Language::Rust) // A safe default
            }
        }
    }
}

/// A choice of language at launch
#[derive(Default, ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LangChoice {
    /// Auto-detect the language from paths (take rust if no path provided)
    #[default]
    Auto,
    /// Should also work for C, and maybe others
    Rust,
    /// It should work, but I didn't do much Java in recent years
    Java,
    /// No idea whether it works for TypeScript
    Js,
}
