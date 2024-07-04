use clap::{Command, Arg, arg, command};


pub fn build_cli() -> Command {
    command!()
        .author("alexrad@880gmail.com")
        .version("0.1")
        .about("to-do list implementation")
        .subcommand(
            Command::new("add")
                .about("Add a new to-do item")
                .arg(
                    Arg::new("item")
                        .short('i')
                        .long("item")
                        .help("The item to add to the to-do list")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("List all to-do items")
        )
        .arg(arg!(-d --debug "Turns on debugging info"))
        .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
}
