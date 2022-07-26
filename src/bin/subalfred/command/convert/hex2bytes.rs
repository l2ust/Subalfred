// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;

/// Convert the hex to bytes.
#[derive(Debug, Args)]
pub(crate) struct Hex2BytesCmd {
	/// Hex input.
	///
	/// Example: `0x00000000`.
	#[clap(required = true, value_name = "HEX")]
	hex: String,
}
impl Hex2BytesCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { hex } = self;

		println!("{:?}", array_bytes::hex2bytes(hex).map_err(|_| quick_err("invalid hex input"))?);

		Ok(())
	}
}
