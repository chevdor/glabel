mod opts;
use clap::{crate_name, crate_version, Clap};
use env_logger::Env;
use futures::prelude::*;
use glabellib::label::Label;
use glabellib::repo::*;
use glabellib::set::Set;
use hubcaps::{labels::*, Credentials, Github};
use opts::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::{env, fs};
use futures::executor::block_on;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::info!("Running {} v{}", crate_name!(), crate_version!());

	let opts: Opts = Opts::parse();

	match opts.subcmd {
		SubCommand::Get(get_opts) => {
			let pat = env::var("GITHUB_TOKEN").unwrap_or(String::new());
			log::debug!("PAT: {}", if !pat.is_empty() { "SET" } else { "NOT SET " });

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

		SubCommand::Apply(apply_opts) => {
			log::debug!("apply: {:#?}", apply_opts);

			let repo = Repo::from_str(&apply_opts.repository).unwrap();
			let token = env::var("GITHUB_TOKEN").unwrap_or(String::new());
			log::debug!("TOKEN: {}", if !token.is_empty() { "SET" } else { "NOT SET " });
			let github = Github::new(String::from("glabel"), Credentials::Token(token))?;
			let gh_repo = github.repo(repo.owner, repo.repository);
			let labels = gh_repo.labels();

			// first we need to load the set and deserialize it
			let set_file = apply_opts.input;
			let s = fs::read_to_string(set_file)?;
			let set: Set = serde_yaml::from_str(&s)?;
			log::info!("Loaded set {}", set.name);
			// we then login with our write token

			// and write the labels
			set.labels.iter().for_each(move |label| {
				log::debug!("processing label = {:?}", label);
				// TODO: add conversion from glabel label back to hubcaps labels
				let label_options: LabelOptions = LabelOptions {
					name: label.name.clone().into(),
					color: label.color.clone().into(),
					description: label.description.clone().unwrap_or_default(),
				};

				let _ = block_on(labels.create(&label_options));
			})
		}
	};

	Ok(())
}
