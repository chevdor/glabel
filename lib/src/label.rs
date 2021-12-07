use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
	pub name: String,
	pub description: Option<String>,
	pub color: String,
}

impl From<hubcaps::labels::Label> for Label {
	fn from(l: hubcaps::labels::Label) -> Self {
		Self { name: l.name, description: l.description, color: l.color }
	}
}
