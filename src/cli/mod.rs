mod args;

use {
    args::*,
    clap::Parser,
    code_sort::*,
    std::io::Write,
};

/// Run the cli application
pub fn run() -> CsResult<()> {
    let args = Args::parse();

    let stdin = std::io::stdin();
    let reader = stdin.lock();

    let list = List::from_reader(reader)?;

    let window = match (args.around, args.range) {
        (Some(_), Some(_)) => {
            return Err(CsError::RangeAndAround);
        }
        (Some(line), None) => {
            tprint!(
                "line: {} : {}",
                line,
                list.line_by_number(line).unwrap_or("-no line here-"),
            );
            list.window_around_line(line)?
        }
        (None, Some(range)) => list.window_on_line_range(range)?,
        _ => list.into_window(),
    };
    tprintln!("range: {:?}", window.range());

    //{
    //    let mut blocks = window.blocks();
    //    window.sort_blocks(&mut blocks);
    //    tprintln!("blocks:");
    //    for block in blocks {
    //        tprintln!("\n------\n{}", block.content());
    //    }
    //}

    let stdout = std::io::stdout();
    let mut writer = stdout.lock();
    write!(&mut writer, "{}", window.sort())?;

    Ok(())
}
