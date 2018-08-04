use std::path;

pub struct State {
	directory: path::PathBuf
}

impl State {
	pub fn new(directory: &str) -> State {
		State {
			directory: path::PathBuf::from(directory)
		}
	}

	pub fn get_directory(&self) -> String {
		self.directory.to_str().unwrap().to_string()
	}

	pub fn push_directory(&mut self, path: String) {
		let new_path = path::PathBuf::from(path);
		self.directory.push(new_path);
	}

	pub fn pop_directory(&mut self) {
		self.directory.pop();
	}
}