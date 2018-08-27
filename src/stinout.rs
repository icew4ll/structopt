#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use std::io::{self, Read, Write};
use std::fs;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input file, default to stdin.
    #[structopt(short = "i", parse(from_os_str))]
    input: Option<PathBuf>,
    /// Output file, default to stdout.
    #[structopt(short = "o", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    run(Opt::from_args()).unwrap();
}

fn run(opt: Opt) -> io::Result<()> {
    match (opt.input, opt.output) {
        (None, None) => cat(io::stdin(), io::stdout())?,
        (Some(i), Some(o)) => cat(fs::File::open(i)?, fs::File::create(o)?)?,
        (None, Some(o)) => cat(io::stdin(), fs::File::create(o)?)?,
        (Some(i), None) => cat(fs::File::open(i)?, io::stdout())?,
    }
    Ok(())
}

fn cat<R: Read, W: Write>(reader: R, mut writer: W) -> io::Result<()> {
    for b in reader.bytes() {
        writer.write(&[b?])?;
    }
    Ok(())
}
