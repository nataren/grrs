use structopt::StructOpt;
use std::fmt;

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

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
