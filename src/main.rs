use std::env;
use std::process;

use listr::print_usage;
// TODO bring in env vars for LISTR_INTERACTIVE;

fn main() {
    let _args = listr::count_and_collect_args(env::args()).unwrap_or_else(|err| {
        println!("Error: {}", err);
        print_usage();
        process::exit(1);
    });
}
