use std::io::{self, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>,
    #[structopt(short = "r", long = "recurse")]
    recurse: bool,
}

impl Opt {
    fn path(&self) -> &Path {
        self.path
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or_else(|| Path::new("."))
    }
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let stream = io::stdout();
    let mut stream = stream.lock();

    let walker = WalkDir::new(&opt.path()).min_depth(1);
    let walker = if opt.recurse {
        walker
    } else {
        walker.max_depth(1)
    };

    for entry in walker.into_iter().filter_map(Result::ok) {
        writeln!(stream, "{}", entry.path().display())?;
    }

    Ok(())
}
