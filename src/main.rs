use std::io;
use std::io::Write;

use crate::cli::build_cli;
use crate::list::TodoList;
use crate::storage::FileWriter;

mod cli;
mod list;
mod storage;

fn main() {
    let cli_command = build_cli();
    const URL: &str = "./list.json";
    let mut file_writer: FileWriter = FileWriter::new(URL, false)
        .expect("Could not find the file in location");
    let file_content = file_writer.read_file()
        .expect("Cannot read file");

    let mut todo_list: TodoList;
    if file_content.is_empty() {
        todo_list = Default::default();
    } else {
        let deserialized = serde_json::from_str(file_content.as_str())
            .expect("Cannot deserialize the file");
        todo_list = TodoList::new(deserialized);
    }

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let args: Vec<String> = shell_words::split(input).expect("Failed to parse input");

        let matches_result = cli_command.clone().try_get_matches_from(args);

        match matches_result {
            Ok(matches) => {
                match matches.subcommand() {
                    Some(("add", sub_m)) => {
                        if let Some(item) = sub_m.get_one::<String>("item") {
                            todo_list.add_todo_item(item);
                        } else {
                            println!("Item is required for add command.");
                        }
                    }
                    Some(("list", _sub_m)) => {
                        todo_list.list();
                    }
                    Some(("save", _sub_m)) => {
                        let list = todo_list.list();
                        let serialized = serde_json::to_string(list).unwrap();
                        file_writer = FileWriter::new(URL, true).unwrap();
                        file_writer.write_to_file(serialized)
                            .expect("Could not write to file");
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
