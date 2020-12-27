use listr::print_usage;
use std::env;
use std::process;
// TODO: add interactive mode.
// TODO: bring in env vars for LISTR_INTERACTIVE;

fn main() {
    if let Err(e) = listr::run(env::args()) {
        eprintln!("Error: {}", e);
        print_usage();
        process::exit(1);
    };
}
