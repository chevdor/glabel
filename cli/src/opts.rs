use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::path::PathBuf;

/// This utility allows fetching, apply and validating github labels
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// Output as json
	#[clap(short, long, global = true)]
	pub json: bool,

	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Debug, Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Get(GetOpts),
}

/// Get/download labels
#[derive(Debug, Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct GetOpts {
	/// The repo string for now in the form owner/repo such as chevdor/foobar
	#[clap(required = true, index = 1)]
	pub repository: String,

	/// The output filename
	#[clap(short, long, alias("out"), parse(from_os_str))]
	pub output: Option<PathBuf>,
}
