use std::path;

use state;

pub trait Command {
	fn execute(&self, Vec<String>, &mut state::State) -> Option<String>;
}

#[derive(Debug)]
pub struct Echo {}

impl Command for Echo {
	fn execute(&self, params: Vec<String>, state: &mut state::State) -> Option<String> {
		Some(params.join(" "))
	}
}

#[derive(Debug)]
pub struct Pwd {}

impl Command for Pwd {
	fn execute(&self, params: Vec<String>, state: &mut state::State) -> Option<String> {
		Some(state.get_directory())
	}
}

#[derive(Debug)]
pub struct Cd {}

impl Command for Cd {
	fn execute(&self, params: Vec<String>, state: &mut state::State) -> Option<String> {
		let new_path = match params.first() {
			Some(param) => param.clone(),
			None => return None
		};

		if new_path == ".." {
			state.pop_directory()
		} else if new_path == "." {
			// Do nothing
		} else {
			state.push_directory(new_path)
		}

		Some(state.get_directory())
	}
}