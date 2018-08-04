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

		let command: Option<Box<commands::Command>> = match parsed_command.name.as_ref() {
			"echo" => Some(Box::new(commands::Echo{})),
			"pwd" =>  Some(Box::new(commands::Pwd{})),
			"cd" => Some(Box::new(commands::Cd{})),
			_ => None
		};

		let output = match command {
			Some(cmd) => cmd.execute(parsed_command.params, &mut state),
			None => Some("Command not found".to_string())
		};

		match output {
			Some(output_str) => println!("{}", output_str),
			None => println!("")
		}
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