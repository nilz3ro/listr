use std::env;
use std::error::Error;

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
    // TODO: Figure out how to pass this as an error to the top level.
    let command = validate_command(counted_args).expect("Invalid command");

    println!("command parsed: {:?}", command);

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

fn validate_command(args: Vec<String>) -> Option<Command> {
    let mut args = args.into_iter();

    let first_arg = match &args.next().unwrap()[..] {
        "help" => return Some(Command::Help),
        "add" => Some("add"),
        "remove" => Some("remove"),
        "show" => Some("show"),
        _ => return None,
    };

    // we definitely have a valid command now.
    // calling unwrap is safe.
    let add_remove_or_show = first_arg.unwrap();

    // find out if there's a list name.
    let list_name = match args.next() {
        Some(list) => list,
        None => return None,
    };

    // if there's an item add it to the list.
    if let Some(item_name) = args.next() {
        match add_remove_or_show {
            "add" => return Some(Command::AddItemToList(list_name, item_name)),
            "remove" => return Some(Command::RemoveItemFromList(list_name, item_name)),
            "show" | _ => return None,
        }
    }

    // just add the list.
    match add_remove_or_show {
        "add" => Some(Command::AddList(list_name)),
        "remove" => Some(Command::RemoveList(list_name)),
        "show" => Some(Command::ShowList(list_name)),
        _ => None,
    }
}

pub fn print_usage() {
    let usage = "\nlistr usage:          

add <list>                 Adds a list.
remove <list>              Removes a list.
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

            assert_eq!(result.is_none(), true);
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

            assert_eq!(result.is_none(), true);
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
