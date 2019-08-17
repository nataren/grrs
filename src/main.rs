
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "n", long = "pattern")]
    pattern: String,

    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: std::path::PathBuf,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(pattern={}, path={})", self.pattern, self.path.display())
    }
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let l = line.unwrap();
        if l.contains(&args.pattern) {
            println!("{}", l);
        }
    }
    Ok(())
}
