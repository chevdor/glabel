use clap::{crate_authors, crate_version, Parser, Subcommand};
use std::path::PathBuf;

/// This utility allows fetching, apply and validating github labels
#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	// /// Output as json
	// #[clap(short, long, global = true)]
	// pub json: bool,
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Get(GetOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Apply(ApplyOpts),

	#[cfg(feature = "wipe")]
	#[clap(version = crate_version!(), author = crate_authors!())]
	Wipe(WipeOpts),
}

/// Get/download labels from a given repository.
#[derive(Debug, Parser)]
pub struct GetOpts {
	/// The repo string for now in the form owner/repo such as chevdor/foobar
	#[clap(required = true, index = 1)]
	pub repository: String,

	/// The output filename
	#[clap(short, long, alias("out"), parse(from_os_str))]
	pub output: Option<PathBuf>,
}

/// Apply a label set from a given file.
#[derive(Debug, Parser)]
pub struct ApplyOpts {
	/// The repo string for now in the form owner/repo such as chevdor/foobar
	#[clap(required = true, index = 1)]
	pub repository: String,

	/// The filename where your set is stored
	#[clap(required = true, parse(from_os_str), index = 2)]
	pub input: PathBuf,

	/// If you follow good security practices, your GITHUB_TOKEN
	/// should not have write access to your repos. Here since we
	/// need to write labels, we use another variable for the token
	/// with write access. It is highly recommended to pass this as
	/// an Environment variable.
	#[clap(short, long, required = true, env = "TOKEN")]
	pub token: String,

	/// By default, existing labels will NOT be updated.
	/// If you set this flag to true, they will. Beware, there is no
	/// automatic backup so it could be a good idea to run the `get`
	/// command first and make a backup.
	#[clap(short, long)]
	pub replace: bool,

	/// Do not change anything, print only what will be done.
	#[clap(short, long)]
	pub dry_run: bool,
}

#[cfg(feature = "wipe")]
/// Allows wiping all labels in a repo. Definitely not a safe
/// command, this is why should consider twice before using it.
/// If **some** labels are used in your issues, PRs, you probably
/// should NOT be doing a wipe.
#[derive(Parser, Debug)]
pub struct WipeOpts {
	/// The repo string for now in the form owner/repo such as chevdor/foobar
	#[clap(required = true, index = 1)]
	pub repository: String,

	/// If you follow good security practices, your GITHUB_TOKEN
	/// should not have write access to your repos. Here since we
	/// need to write labels, we use another variable for the token
	/// with write access. It is highly recommended to pass this as
	/// an Environment variable.
	#[clap(short, long, required = true, env = "TOKEN")]
	pub token: String,

	/// By default, for your safety, the command will NOT do anything.
	/// If you however pass this flag, there will be no way back :)
	/// It is highly recommended to call `get` first and backup your
	/// labels but keep in mind that it does not save which labels are
	/// applied to PRs and issues. So if you have labels "in use", you
	/// will lose them.
	#[clap(short, long)]
	pub apply: bool,
}
