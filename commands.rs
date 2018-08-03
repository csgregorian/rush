pub trait Command {
	fn execute(&self, Vec<String>);
}

#[derive(Debug)]
pub struct Echo {}

impl Command for Echo {
	fn execute(&self, params: Vec<String>) {
		println!("{}", params.join(" "))
	}
}