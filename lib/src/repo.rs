use std::str::FromStr;

#[derive(Debug)]
pub struct Repo {
	pub owner: String,
	pub repository: String,
}

impl FromStr for Repo {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = s.split('/').collect();
		Ok(Repo { owner: parts[0].into(), repository: parts[1].into() })
	}
}

#[cfg(test)]
mod test_repo {
    use super::*;

    #[test]
    fn test_parse() {
        let repo = Repo::from_str("chevdor/glabel").unwrap();
        assert_eq!("chevdor", repo.owner);
        assert_eq!("glabel", repo.repository);
    }
}
