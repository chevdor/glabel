mod opts;
use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use futures::{executor::block_on, prelude::*};
use glabellib::{label::Label, repo::*, set::Set};
use hubcaps::{labels::*, Credentials, Github};
use opts::*;
use std::error::Error;
use std::{fs::{File, self}, io::prelude::*, str::FromStr, env};
use termion::{color, style};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::info!("Running {} v{}", crate_name!(), crate_version!());

	let opts: Opts = Opts::parse();
	let pat = env::var("GITHUB_TOKEN").unwrap_or_default();
	log::debug!("PAT: {}", if !pat.is_empty() { "SET" } else { "NOT SET " });

	match opts.subcmd {
		SubCommand::Get(get_opts) => {
			log::debug!("get: {:#?}", get_opts);
			let input_repo = Repo::from_str(&get_opts.repository).unwrap();
			let github = Github::new(String::from("glabel"), Credentials::Token(pat))?;

			log::debug!("connected");

			// TODO: there is the same block below, factorize that away !
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
				file.write_all(yml.as_bytes())?;
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
			let token = env::var("GITHUB_TOKEN").unwrap_or_default();
			log::debug!("TOKEN: {}", if !token.is_empty() { "SET" } else { "NOT SET " });

			let github = Github::new(String::from("glabel"), Credentials::Token(token))?;

			let gh_repo = github.repo(repo.owner, repo.repository);
			let labels = gh_repo.labels().iter().map(|label| Label::from(label.unwrap())).collect::<Vec<_>>().await;

			let gh_labels = gh_repo.labels();

			// Tests to find all PRs incl. labels, in a repo
			// github
			// 	.search()
			// 	.issues()
			// 	.iter("user:softprops", &SearchIssuesOptions::builder().per_page(100).build())
			// 	.try_for_each(|issue| async move {
			// 		println!("{} {:?}", issue.title, issue.pull_request);
			// 		Ok(())
			// 	})
			// 	.await?;

			// first we need to load the set and deserialize it
			let set_file = apply_opts.input;
			let s = fs::read_to_string(set_file)?;
			let set: Set = serde_yaml::from_str(&s)?;
			log::info!("Loaded set {}", set.name);
			// we then login with our write token

			println!("Processing {} labels", set.labels.len());
			println!("It should take around {:0.2} seconds or less", set.labels.len() as f32 / 3.6);
			let mut i: usize = 0;
			let max = set.labels.len();
			// and write the labels
			set.labels.iter().for_each(move |label| {
				i += 1;

				log::debug!("processing label = {:?}", label);
				// TODO: add conversion from glabel label back to hubcaps labels
				let label_options: LabelOptions = LabelOptions {
					name: label.name.clone(),
					color: label.color.clone(),
					description: label.description.clone().unwrap_or_default(),
				};

				print!("Processing {:03}/{:03}: {: <24}\t", i, max, label.name);
				let hit = labels.iter().find(|x| x.name == label.name);

				if let Some(hit_label) = hit {
					if hit_label != &label_options {
						if apply_opts.replace {
							if !apply_opts.dry_run {
								let _ = block_on(gh_labels.update(&label.name, &label_options));
								print!("{}updated{}", color::Fg(color::Yellow), style::Reset);
							} else {
								print!("{}updated [dry run]{}", color::Fg(color::Green), style::Reset);
							}
						} else {
							print!("{}replace not allowed{}", color::Fg(color::Green), style::Reset);
						}
					} else {
						print!("{}skipped, not needed{}", color::Fg(color::Cyan), style::Reset);
					}
				} else if !apply_opts.dry_run {
					let _ = block_on(gh_labels.create(&label_options));
					print!("{}created{}", color::Fg(color::Green), style::Reset);
				} else {
					print!("{}created [dry run]{}", color::Fg(color::Green), style::Reset);
				}

				println!();
			})
		}

		#[cfg(feature = "wipe")]
		SubCommand::Wipe(wipe_opts) => {
			log::debug!("wipe: {:#?}", wipe_opts);

			let repo = Repo::from_str(&wipe_opts.repository).unwrap();
			let token = env::var("GITHUB_TOKEN").unwrap_or_default();
			log::debug!("TOKEN: {}", if !token.is_empty() { "SET" } else { "NOT SET " });

			let github = Github::new(String::from("glabel"), Credentials::Token(token))?;
			let gh_repo = github.repo(repo.owner, repo.repository);
			let labels = gh_repo.labels().iter().map(|label| Label::from(label.unwrap())).collect::<Vec<_>>().await;
			let gh_labels = gh_repo.labels();

			println!("Deleting {} label(s) from {}", labels.len(), wipe_opts.repository);
			labels.iter().for_each(|label| {
				if wipe_opts.apply {
					println!(" - Processing {: <24}\t{}Deleting{}", label.name, color::Fg(color::Red), style::Reset);

					// TODO: Check if the label is in use
					let _ = block_on(gh_labels.delete(&label.name));
				} else {
					println!(" - Processing {: <24}\t{}Dry run{}", label.name, color::Fg(color::Green), style::Reset);
				}
			});
		}
	};

	Ok(())
}
