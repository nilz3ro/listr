use std::error::Error;

pub fn run(_counted_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
// move to lib.
pub fn count_and_collect_args<T>(mut args: T) -> Result<Vec<String>, &'static str>
where
    T: Iterator<Item = String>, // This makes it work with env::Args or Vec<String> (for testing)
{
    // The first argument is the command called to run
    // the program, and we don't need that.
    args.next();

    // Collect the rest of the arguments.
    let collected: Vec<String> = args.collect();

    match collected.len() {
        1 | 2 | 4 => Ok(collected),
        _ => Err("Wrong number of arguments specified"),
    }
}

pub fn print_usage() {
    let usage = "\nlistr usage:          

add <list> -------------------------------- Adds a list.
remove <list> -------------------------- Removes a list.
show <list> -------------- Prints a list to the console.
add <item> to <list> -------- Adds a new item to a list.
remove <item> from <list> - Removes an item from a list.
-h | --help ------------------ Prints this help message.

";

    println!("{}", usage);
}
// fn run() -> Result<(), Box<dyn Error>> {}

// fn build_command()
