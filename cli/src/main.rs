mod opts;
use clap::{crate_name, crate_version, Clap};
use env_logger::Env;
use futures::prelude::*;
use glabellib::label::Label;
use glabellib::repo::*;
use glabellib::set::Set;
use hubcaps::{Credentials, Github};
use opts::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::info!("Running {} v{}", crate_name!(), crate_version!());

	let pat = env::var("GITHUB_TOKEN").unwrap_or(String::new());
	log::debug!("PAT: {}", if !pat.is_empty() { "SET" } else { "NOT SET " });
	let opts: Opts = Opts::parse();

	match opts.subcmd {
		SubCommand::Get(get_opts) => {
			log::debug!("get: {:#?}", get_opts);
			let input_repo = Repo::from_str(&get_opts.repository).unwrap();
			let github = Github::new(String::from("glabel"), Credentials::Token(pat))?;

			log::debug!("connected");

			let labels = github
				.repo(input_repo.owner, input_repo.repository)
				.labels()
				.iter()
				.map(|label| Label::from(label.unwrap()))
				.collect::<Vec<_>>()
				.await;

			if let Some(file) = get_opts.output {
				let description = format!("Import from {}", get_opts.repository);
				let set = Set::new(&get_opts.repository, Some(description), labels);
				let yml = serde_yaml::to_string(&set).unwrap();
				let mut file = File::create(file)?;
				file.write(yml.as_bytes())?;
			} else {
				labels.iter().for_each(|label| {
					println!(
						" - {: <24}\t[{}]: {}",
						label.name,
						label.color,
						label.description.as_ref().unwrap_or(&String::from("n/a"))
					);
				})
			}
		}
	};

	Ok(())
}
