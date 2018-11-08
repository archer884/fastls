use std::io::Write;
use std::{env, io};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let path = env::args().nth(1);
    let path = path.as_ref().map(AsRef::as_ref).unwrap_or(".");

    let stream = io::stdout();
    let mut stream = stream.lock();

    let walker = WalkDir::new(path).min_depth(1).max_depth(1);

    for entry in walker.into_iter().filter_map(Result::ok) {
        writeln!(stream, "{}", entry.path().display())?;
    }

    Ok(())
}
