use {
    clap::Parser,
    codesort::*,
    std::path::PathBuf,
};

/// dump to stdout informations about the sorting of the given file
/// at the specified location
#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Args {
    /// line number (1-based) around which to sort
    #[clap(long)]
    pub sort_around: Option<LineNumber>,

    /// Path to the file(s)
    pub path: PathBuf,
}

fn main() -> CsResult<()> {
    let args = Args::parse();
    let loc_list = LocList::read_file(&args.path, Language::Rust)?;
    let Some(sort_around) = args.sort_around else {
        loc_list.print_debug(" WHOLE FILE ");
        return Ok(());
    };
    let focused = loc_list.focus_around_line_number(sort_around)?;
    focused.print_debug();
    let blocks = focused.clone().focus.into_blocks();
    for (i, block) in blocks.iter().enumerate() {
        block.print_debug(&format!(" BLOCK {i} "));
    }
    let loc_list = focused.sort();
    println!("------------ RESULT ------------");
    print!("{}", loc_list);
    Ok(())
}
