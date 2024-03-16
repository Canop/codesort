use {
    code_sort::*,
    std::io::Write,
};

fn main() -> CsResult<()> {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let list = List::from_reader(reader)?;
    let window = list.into_window();

    let stdout = std::io::stdout();
    let mut writer = stdout.lock();
    write!(&mut writer, "{}", window.sort())?;

    Ok(())
}
