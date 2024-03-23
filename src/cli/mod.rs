mod args;

use {
    args::*,
    clap::Parser,
    codesort::*,
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
        println!("codesort {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let src = args.src.as_ref().or(args.file.as_ref());
    let dst = args.dst.as_ref().or(args.file.as_ref());
    let lang = args.lang();

    // Read input
    let list = if let Some(src) = src {
        let file = fs::File::open(src)?;
        let reader = BufReader::new(file);
        LocList::read(reader, lang)?
    } else {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        LocList::read(reader, lang)?
    };

    // Focus the list to the area to sort
    let focused = match (args.around, args.range) {
        (Some(_), Some(_)) => {
            return Err(CsError::RangeAndAround);
        }
        (Some(line), None) => list.focus_around_line_number(line)?,
        (None, Some(range)) => list.focus(range)?,
        _ => list.focus_all()?,
    };

    let sorted_list = focused.sort();

    // Write output
    if let Some(dst) = dst {
        let file = fs::File::create(dst)?;
        let mut writer = std::io::BufWriter::new(file);
        write!(&mut writer, "{}", sorted_list)?;
    } else {
        print!("{}", sorted_list);
    }

    Ok(())
}
