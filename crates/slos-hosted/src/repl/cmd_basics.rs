//! Basic command set

use super::Context;
use anyhow::Result;

/// Echo the given arguments back to the console
pub fn cmd_echo(_context: &mut Context, args: &[String]) -> Result<()> {
	println!("{}", args.join(" "));
	Ok(())
}
