use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_str))]
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::from_args();
    println!("pattern: {}\npath: {:?}", args.pattern, args.path);
}
