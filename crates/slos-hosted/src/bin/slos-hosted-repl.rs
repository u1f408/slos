use anyhow::Result;
use slos_hosted::repl;

fn main() -> Result<()> {
	if let Err(e) = repl::run_repl(repl::Context::new(), repl::default_cmds()) {
		return Err(e);
	}

	Ok(())
}
