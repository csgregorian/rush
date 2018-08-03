use std::io;

fn main() {
	loop {
		println!("> ");

		let input = get_input();
		let parsed_command = make_command(input);

		let command = match parsed_command.name.as_ref() {
			"echo" => Some(commands::Echo{}),
			_ => None
		};

		match command {
			Some(cmd) => cmd.execute(parsed_command.params),
			None => println!("Command not found")
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