use super::Context;
use anyhow::Result;

pub fn cmd_echo(_context: &mut Context, args: &[String]) -> Result<()> {
	println!("{}", args.join(" "));
	Ok(())
}
