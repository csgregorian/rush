use state::State;

pub trait Command {
	fn execute(&self, Vec<String>, state::State);
}

#[derive(Debug)]
pub struct Echo {}

impl Command for Echo {
	fn execute(&self, params: Vec<String>, state: State) {
		println!("{}", params.join(" "))
	}
}

#[derive(Debug)]
pub struct Pwd {}

impl Command for Pwd {
	fn execute(&self, params: Vec<String>, state: State) {
		println!("{}", params.join(" "))
	}
}