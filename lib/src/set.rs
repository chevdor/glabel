use crate::label::Label;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Set {
	name: String,
	description: Option<String>,

	pub labels: Vec<Label>,
}

impl Set {
	pub fn new(name: &str, description: Option<String>, labels: Vec<Label>) -> Self {
		Self { name: name.into(), description, labels }
	}
}

#[cfg(test)]
mod test_set {
	use super::*;

	#[test]
	fn test_serialize() {
		let set = Set::new("test", None, Vec::new());
		let yml = serde_yaml::to_string(&set).unwrap();
		println!("{}", yml);
	}
}
