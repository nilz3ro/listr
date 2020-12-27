use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::{env, io::ErrorKind};

#[derive(Debug, PartialEq)]
enum Command {
    AddList(String),
    RemoveList(String),
    ShowList(String),
    AddItemToList(String, String),
    RemoveItemFromList(String, String),
    Help,
}

pub fn run(args: env::Args) -> Result<(), Box<dyn Error>> {
    let counted_args = count_and_collect_args(args)?;
    // TODO: Figure out how to pass this as an error to caller using '?'.
    let command = validate_command(counted_args)?;

    handle_command(command)?;

    Ok(())
}

fn count_and_collect_args<T>(mut args: T) -> Result<Vec<String>, &'static str>
where
    T: Iterator<Item = String>,
{
    // The first argument is the command called to run
    // the program, and we don't need that.
    args.next();

    // Collect the rest of the arguments.
    let collected: Vec<String> = args.collect();

    match collected.len() {
        1 | 2 | 3 => Ok(collected),
        _ => Err("Wrong number of arguments specified."),
    }
}

fn validate_command(args: Vec<String>) -> Result<Command, &'static str> {
    let mut args = args.into_iter();

    let first_arg = match &args.next().unwrap()[..] {
        "help" => return Ok(Command::Help),
        "add" => Some("add"),
        "remove" => Some("remove"),
        "show" => Some("show"),
        _ => return Err("Invalid Argument."),
    };

    // we definitely have a valid command now.
    // calling unwrap is safe.
    let add_remove_or_show = first_arg.unwrap();

    // find out if there's a list name.
    let list_name = match args.next() {
        Some(list) => list,
        None => return Err("Please supply a list name."),
    };

    // TODO: Fix code smell. find a way to match the argument
    // once and build comands from there.

    // if there's an item add it to the list.
    if let Some(item_name) = args.next() {
        match add_remove_or_show {
            "add" => return Ok(Command::AddItemToList(list_name, item_name)),
            "remove" => return Ok(Command::RemoveItemFromList(list_name, item_name)),
            "show" | _ => return Err("Show only takes one argument: <list>."),
        }
    }

    // just add the list.
    match add_remove_or_show {
        "add" => Ok(Command::AddList(list_name)),
        "remove" => Ok(Command::RemoveList(list_name)),
        "show" => Ok(Command::ShowList(list_name)),
        _ => Err("Invalid argument."),
    }
}

fn handle_command(command: Command) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Help => {
            print_usage();
            Ok(())
        }
        Command::ShowList(list) => show_list(list),
        Command::AddList(list) => add_list(list),
        Command::AddItemToList(list, item) => {
            println!("AddItemToList list: {}, item:{}", list, item);
            Ok(())
        }
        Command::RemoveList(list) => {
            println!("RemoveList: {}", list);
            Ok(())
        }
        Command::RemoveItemFromList(list, item) => {
            println!("RemoveItemFromList: list: {}, item: {}", list, item);
            Ok(())
        }
    }
}

fn ensure_db_file_exists() -> Result<(), std::io::Error> {
    // TODO: make sure the db file defaults to the user's home dir.
    // TODO: read an environment variable that lets the user control
    // the path to the db file.
    match File::open("./listr_db.json") {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                // create the file
                let mut f = File::create("./listr_db.json")?;
                // add empty json object.
                f.write(b"{}")?;
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

fn add_list(list: String) -> Result<(), Box<dyn Error>> {
    ensure_db_file_exists()?;

    let mut file = File::open("./listr_db.json")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut list_map: HashMap<String, Vec<String>> = serde_json::from_str(&contents)?;

    list_map.insert(list, vec![]);

    let output = serde_json::to_string(&list_map)?;
    let mut file = File::create("./listr_db.json")?;

    file.write(output.as_bytes())?;

    Ok(())
}

fn show_list(list: String) -> Result<(), Box<dyn Error>> {
    ensure_db_file_exists()?;

    let mut file = File::open("./listr_db.json")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let list_map: HashMap<String, Vec<String>> = serde_json::from_str(&contents)?;

    match list_map.get(&list) {
        Some(list_contents) => {
            println!("{}: {:?}", list,  list_contents);
            Ok(())
        }
        None => {
            println!("no list named '{}' exists", list);
            Ok(())
        }
    }
}

pub fn print_usage() {
    let usage = "\nlistr usage:          

add <list>                 Adds a listremove <list>              Removes a list.
show <list>                Prints a list to the console.
add <list> <item>          Adds a new item to a list.
remove <list> <item>       Removes an item from a list.
help                       Prints this help message.\n";

    println!("{}", usage);
}

#[cfg(test)]
mod test {

    mod count_and_collect {
        use crate::*;
        #[test]
        fn without_args() {
            let arg0 = String::from("/usr/bin/whatever/listr");

            let collected = count_and_collect_args(vec![arg0].into_iter());
            assert_eq!(collected, Err("Wrong number of arguments specified."));
        }

        #[test]
        fn with_one_arg() {
            let arg0 = String::from("/usr/bin/whatever/listr");
            let arg1 = String::from("one");
            let collected = count_and_collect_args(vec![arg0, arg1].into_iter());

            assert_eq!(collected, Ok(vec![String::from("one")]));
        }

        #[test]
        fn two_args() {
            let arg0 = String::from("/usr/bin/whatever/listr");
            let arg1 = String::from("one");
            let arg2 = String::from("two");
            let collected = count_and_collect_args(vec![arg0, arg1, arg2].into_iter());

            assert_eq!(
                collected,
                Ok(vec![String::from("one"), String::from("two")])
            );
        }

        #[test]
        fn with_three_args() {
            let arg0 = String::from("/usr/bin/whatever/listr");
            let arg1 = String::from("one");
            let arg2 = String::from("two");
            let arg3 = String::from("three");
            let collected = count_and_collect_args(vec![arg0, arg1, arg2, arg3].into_iter());

            assert_eq!(
                collected,
                Ok(vec![
                    String::from("one"),
                    String::from("two"),
                    String::from("three")
                ])
            );
        }

        #[test]
        fn with_four_args() {
            let arg0 = String::from("/usr/bin/whatever/listr");
            let arg1 = String::from("one");
            let arg2 = String::from("two");
            let arg3 = String::from("three");
            let arg4 = String::from("four");
            let collected = count_and_collect_args(vec![arg0, arg1, arg2, arg3, arg4].into_iter());

            assert_eq!(collected, Err("Wrong number of arguments specified."));
        }
    }

    mod validate_command {
        use crate::validate_command;
        use crate::Command;

        #[test]
        fn invalid_argument() {
            let args = vec![String::from("sandwich")];
            let result = validate_command(args);

            assert_eq!(result.is_err(), true);
        }
        #[test]
        fn help() {
            let args = vec![String::from("help")];
            let result = validate_command(args).unwrap();

            assert_eq!(result, Command::Help);
        }
        #[test]
        fn add_without_list() {
            let args = vec![String::from("add")];
            let result = validate_command(args);

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn add_with_list() {
            let args = vec![String::from("add"), String::from("books")];
            let result = validate_command(args).unwrap();

            assert_eq!(result, Command::AddList(String::from("books")));
        }

        #[test]
        fn add_with_list_and_item() {
            let args = vec![
                String::from("add"),
                String::from("books"),
                String::from("eragon"),
            ];
            let result = validate_command(args).unwrap();

            assert_eq!(
                result,
                Command::AddItemToList(String::from("books"), String::from("eragon"))
            );
        }
    }
}
