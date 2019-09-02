use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_str))]
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::from_args();
    let f = File::open(&args.path).expect("Could not find file");
    let reader = BufReader::new(f);
    /*
    Notes:
     * `f` and `reader` will go out of scope and close without leaking
     * reader.lines() comes from the `BufReader` trait
     * `BufReader` must be in scope - that is have a use command, to work
     * `lines()` returns an interator of `std::result::results`
     * `unwrap_or_default()` returns the value associated with an Ok(T) result, or the
    empty string. This effectively sinks errors.
     */
    for line in reader.lines().map(|l| l.unwrap_or_default()) {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
