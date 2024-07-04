use std::io;
use std::io::Write;

use crate::cli::build_cli;
use crate::list::TodoList;

mod cli;
mod list;

fn main() {
    let cli_command = build_cli();
    let mut todo_list: TodoList = Default::default();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        // Split the input into arguments
        let args: Vec<String> = shell_words::split(input).expect("Failed to parse input");

        // Parse the arguments
        let matches_result = cli_command.clone().try_get_matches_from(args);

        match matches_result {
            Ok(matches) => {
                match matches.subcommand() {
                    Some(("add", sub_m)) => {
                        if let Some(item) = sub_m.get_one::<String>("item") {
                            // println!("Add command executed with item: {}", item);
                            todo_list.add_todo_item(item);
                        } else {
                            println!("Item is required for add command.");
                        }
                    }
                    Some(("list", _sub_m)) => {
                        // println!("{:?}", todo_list.list())
                        todo_list.list();
                    }
                    _ => {
                        println!("Unknown command");
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
