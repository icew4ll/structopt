// setup {{{
//https://rust.libhunt.com/docopt-rs-alternatives
#[macro_use]
extern crate serde_derive;
extern crate docopt;
use docopt::Docopt;
// }}}
// Docopt {{{
const USAGE: &'static str = "
Rust docopt cli
Usage:
    str <command> [<args>...]
    str [options]
Options:
    -h, --help       Display this message
    -V, --version    Print version info and exit
    --list           List installed commands
    -v, --verbose    Use verbose output
See 'str help <command>' for more information on a specific command.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_command: Option<Command>,
    arg_args: Vec<String>,
}

#[derive(Debug, Deserialize)]
enum Command {
    A,
}
// }}}
// main {{{
fn main() {
    // print args
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.options_first(true).deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args.arg_args[0]);
}
// }}}
