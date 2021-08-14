use anyhow::Result;
use super::Context;

pub fn cmd_echo(_context: &mut Context, args: &[String]) -> Result<()> {
    println!("{}", args.join(" "));
    Ok(())
}
