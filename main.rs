use std::io;

fn main() {
	loop {
		println!("Enter commands:");

		let input = get_input();
		let command = make_command(input);
		println!("{:?}", command);
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
struct Command {
	name: String,
	params: Vec<String>,
}

fn make_command(input_string: String) -> Command {
	let mut input = input_string.split_whitespace();
	let name = match input.next() {
		Some(word) => word.to_string(),
		None => panic!("wat"),
	};
	let mut params = vec![];

	for word in input {
		params.push(word.to_string())
	}

	Command {
		name: name,
		params: params,
	}
}