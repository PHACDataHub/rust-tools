use rusty_ulid::Ulid;
use clap::Parser;

/// Simple ULID generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// truncated - first 10 characters of the ulid (i.e. the timestamp)
    #[arg(short, long, default_value_t = false)]
    short_ulid: bool,

    /// Upper case by default, lower is an option
    #[arg(short, long, default_value_t = false)]
    lowercase: bool,
}
fn main() {
    let Args { short_ulid, lowercase } = Args::parse();
    let mut ulid = Ulid::generate().to_string();

    if lowercase {
        ulid = ulid.to_lowercase();
    }

    if short_ulid {
        ulid.drain(11..);
    }

    println!("{}", ulid);
}