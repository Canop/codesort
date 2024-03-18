use {
    clap::Parser,
    code_sort::*,
    std::path::PathBuf,
};

/// Sort code blocks - see https://github.com/Canop/code-sort
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Sort the block around this 1-based line number
    #[arg(long)]
    pub around: Option<LineNumber>,

    /// Sort this start:end range of 1 based lines, both ends included
    #[arg(long)]
    pub range: Option<LineNumberRange>,

    /// The language of the code to sort (default: rust)
    /// Possible values: rust, java, javascript
    #[arg(short, long)]
    pub lang: Option<Language>,

    /// Path to a file to sort (if not provided, will read from stdin)
    #[arg(long)]
    pub src: Option<PathBuf>,

    /// where to write after sort (if not provided, will write to stdout)
    #[arg(long)]
    pub dst: Option<PathBuf>,

    /// file to sort in place (shortcut for --src and --dst)
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}
