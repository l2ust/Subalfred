// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred_core::cargo;

/// Update the workspace members' crate version.
#[derive(Debug, Args)]
pub(crate) struct UpdateCmd {
	/// Path to the root `Cargo.toml`.
	///
	/// If `Cargo.toml` wasn't given, Subalfred will search it under the given path.
	#[arg(long, value_name = "PATH", default_value = "./Cargo.toml")]
	manifest_path: String,
	/// Target version.
	#[arg(required = true, value_name = "VERSION")]
	version: String,
}
impl UpdateCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { manifest_path, version } = self;

		cargo::update_member_versions(manifest_path, version).await?;

		Ok(())
	}
}
