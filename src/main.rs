use std::fmt;
use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

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

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
