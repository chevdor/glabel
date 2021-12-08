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

impl PartialEq for Label {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.description == other.description && self.color == other.color
	}
}

impl PartialEq<hubcaps::labels::LabelOptions> for Label {
	fn eq(&self, other: &hubcaps::labels::LabelOptions) -> bool {
		self.name == other.name && self.description == Some(other.description.clone()) && self.color == other.color
	}
}
