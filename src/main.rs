use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead,BufReader};
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::iter::Iterator;
use log::{info,warn};

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_str))]
    path: std::path::PathBuf
}

fn find_matches(lines: impl IntoIterator<Item=String> , pattern: &str, mut writer: impl std::io::Write) -> ()
{
    for line in lines.into_iter() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Err(err) => warn!("could not write the line: {}", err),
                Ok(_) => ()
            }
        }
    }
}

#[test]
fn can_find_match() {
    let mut result = Vec::new();
    let test = b"abc\ndef\nghi\njkl\nefvincent\nfoobar";
    let lines = test.lines().map(|l| l.unwrap());
    find_matches(lines, "ef", &mut result);
    assert_eq!(result, b"def\nefvincent\n");
}

fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    info!("Starting up!");

    let args = Cli::from_args();
    info!("args: {:?}", args);

    // Note: `with_context` is a trait (extension) that turns an Err() result into
    // an error type with a human readable context. The ? operator matches on error
    // and causes main() to return the Result<> type we see in the decl. the main
    // program must therefore return Ok(()). Note the inner type () is like unit, and
    // there is no semicolon on the returned expression line in the function. Could also
    // have used the keyword return Ok(()).
    let f = File::open(&args.path)
        .with_context(|_| format!("could not read file {:?}", &args.path))?;
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

    find_matches(reader.lines().map(|l| l.unwrap_or_default()), &args.pattern, std::io::stdout());

    Ok(())
}
