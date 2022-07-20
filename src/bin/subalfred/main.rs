#![feature(concat_idents)]
#![warn(missing_docs)]

//! TODO: doc

/// Useful tools set for development.
mod prelude {
	pub use ::std::result::Result as StdResult;

	pub use anyhow::Result;

	// std
	use std::fmt::Debug;
	// crates.io
	use anyhow::Error;

	pub fn quick_err<E>(e: E) -> Error
	where
		E: Debug,
	{
		anyhow::anyhow!("{e:?}")
	}
}
use prelude::Result;

mod cli;
use cli::Cli;

mod command;

// #[tokio::main]
fn main() -> Result<()> {
	Cli::new().run()
}
