mod args;

use {
    args::*,
    clap::Parser,
    code_sort::*,
    std::{
        fs,
        io::{
            BufReader,
            Write,
        },
    },
};

/// Run the cli application
pub fn run() -> CsResult<()> {
    let args = Args::parse();

    if args.help {
        args.print_help();
        return Ok(());
    }

    if args.version {
        println!("code-sort {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let src = args.src.as_ref().or(args.file.as_ref());
    let dst = args.dst.as_ref().or(args.file.as_ref());
    let lang = args.lang();

    tprintln!("lang: {:?}", lang);

    // Read input
    let list = if let Some(src) = src {
        let file = fs::File::open(src)?;
        let reader = BufReader::new(file);
        List::from_reader(reader, lang)?
    } else {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        List::from_reader(reader, lang)?
    };

    // Determine the window to sort
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
    tprintln!("range to sort: {:?}", window.range());

    // Sort
    #[cfg(feature = "explain")]
    {
        let mut blocks = window.blocks()?;
        window.sort_blocks(&mut blocks);
        tprintln!("===== blocks:");
        for block in blocks {
            tprintln!("\n------\n{}", block.content());
        }
        tprintln!("=====");
    }
    let sorted_list = window.sort()?;

    // Write output
    if let Some(dst) = dst {
        let file = fs::File::create(dst)?;
        let mut writer = std::io::BufWriter::new(file);
        write!(&mut writer, "{}", sorted_list)?;
    } else {
        let stdout = std::io::stdout();
        let mut writer = stdout.lock();
        write!(&mut writer, "{}", sorted_list)?;
    }

    Ok(())
}
