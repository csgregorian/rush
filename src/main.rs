use std::io;
use std::io::Write;
use std::option::Option;

mod commands;
mod state;

use commands::Command;

fn main() {
    let mut state = state::State::new("~/");

    loop {
        print!("> ");
        io::stdout().flush();

        let input = get_input();
        let parsed_command = make_command(input);
        let command = get_command(parsed_command.name);

        let output = match command {
            Some(cmd) => run_command(cmd, parsed_command.params, &mut state),
            None => "Command not found".to_string()
        };

        println!("{}", output);
    }
}

fn get_input() -> String {
    let mut input_string = String::new();
    match io::stdin().read_line(&mut input_string) {
        Ok(_) => {
            return input_string;
        }
        Err(error) => {
            println!("{}", error);
            panic!();
        }
    }
}

#[derive(Debug)]
struct ParsedCommand {
    name: String,
    params: Vec<String>,
}

fn make_command(input_string: String) -> ParsedCommand {
    let mut input = input_string.split_whitespace();

    let name = match input.next() {
        Some(word) => word.to_string(),
        None => panic!("wat"),
    };
    let mut params = vec![];

    for word in input {
        params.push(word.to_string())
    }

    ParsedCommand {
        name: name,
        params: params,
    }
}

fn get_command(name: String) -> Option<Box<commands::Command>> {
    match name.as_ref() {
        "echo" => Some(Box::new(commands::Echo{})),
        "pwd" =>  Some(Box::new(commands::Pwd{})),
        "cd" => Some(Box::new(commands::Cd{})),
        "assign" => Some(Box::new(commands::Assign{})),
        _ => None
    }
}

fn run_command(cmd: Box<Command>, params: Vec<String>, state: &mut state::State) -> String {
    match cmd.execute(params, state) {
        Some(output) => output,
        None => String::new()
    }
}