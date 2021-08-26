#[macro_use]
extern crate slos_log;

use anyhow::Result;
use log::LevelFilter;
use std::path::PathBuf;
use structopt::StructOpt;

use slos_hosted::host;
use slos_hosted::repl;

#[derive(Debug, StructOpt)]
#[structopt(name = "slos-hosted")]
struct Opt {
	#[structopt(subcommand)]
	cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
	/// Run slOS in hosted mode
	Kernel {
		/// Arguments to pass to the hosted kernel
		#[structopt(long)]
		kargs: Option<String>,

		/// Path to a compatible archive to mount as the root filesystem
		#[structopt(long)]
		rootfs: Option<PathBuf>,
	},

	/// Run the slos-hosted repl
	Repl,
}

impl Command {
	fn run(&self) -> Result<()> {
		match self {
			Self::Repl => match repl::run_repl(repl::Context::new(), repl::default_cmds()) {
				Ok(_) => Ok(()),
				Err(e) => Err(e),
			},

			Self::Kernel { kargs, rootfs, .. } => {
				host::init(
					kargs.clone().unwrap_or_else(|| String::new()),
					rootfs.clone(),
				)?;

				host::run_kernel()
			}
		}
	}
}

fn main() -> Result<()> {
	env_logger::Builder::new()
		.format_timestamp_nanos()
		.filter_module("slos_hosted::host::interrupts", LevelFilter::Info)
		.parse_default_env()
		.try_init()?;

	let opt = Opt::from_args();
	trace!("opt={:?}", opt);

	opt.cmd.run()
}
