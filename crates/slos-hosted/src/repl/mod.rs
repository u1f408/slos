use anyhow::{bail, Result};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

pub mod context;
pub use self::context::Context;

pub mod cmd_basics;
pub mod cmd_filesystem;

pub type Callback<T> = fn(&mut T, &[String]) -> Result<()>;

pub fn default_cmds() -> HashMap<String, HashMap<String, Box<Callback<Context>>>> {
	let mut categories = HashMap::new();

	categories.insert(String::from("b"), {
		let mut basics = HashMap::new();
		basics.insert(
			String::from("echo"),
			Box::new(cmd_basics::cmd_echo as Callback<Context>),
		);
		basics
	});

	categories.insert(String::from("fs"), {
		let mut fs = HashMap::new();
		fs.insert(
			String::from("path-normalize"),
			Box::new(cmd_filesystem::cmd_path_normalize as Callback<Context>),
		);
		fs.insert(
			String::from("path-split"),
			Box::new(cmd_filesystem::cmd_path_split as Callback<Context>),
		);
		fs.insert(
			String::from("path-join"),
			Box::new(cmd_filesystem::cmd_path_join as Callback<Context>),
		);
		fs.insert(
			String::from("mount-list"),
			Box::new(cmd_filesystem::cmd_mount_list as Callback<Context>),
		);
		fs.insert(
			String::from("mount-new-memoryfs"),
			Box::new(cmd_filesystem::cmd_mount_new_memoryfs as Callback<Context>),
		);
		fs.insert(
			String::from("file-read"),
			Box::new(cmd_filesystem::cmd_file_read as Callback<Context>),
		);
		fs.insert(
			String::from("file-write-test"),
			Box::new(cmd_filesystem::cmd_file_write_test as Callback<Context>),
		);
		fs
	});

	categories
}

pub fn run_repl<'a, T>(
	mut context: T,
	cmds: HashMap<String, HashMap<String, Box<Callback<T>>>>,
) -> Result<T> {
	let mut rl = Editor::<()>::new();

	loop {
		let line = rl.readline("slos-hosted> ");
		match line {
			Ok(line) => {
				let line = if let Some(l) = shlex::split(&line) {
					l
				} else {
					eprintln!("invalid input");
					continue;
				};

				let mut cmd = String::new();
				let (cat, args) = if let Some((cat, args)) = line.split_first() {
					if args.len() > 0 {
						if let Some((xcmd, xargs)) = args.split_first() {
							cmd = xcmd.clone();
							(cat, xargs)
						} else {
							(cat, args)
						}
					} else {
						(cat, (&[] as &[String]))
					}
				} else {
					continue;
				};

				match cat.as_str() {
					"quit" | "exit" => {
						break;
					}

					"?" | "help" => {
						for (category, ncat) in cmds.iter() {
							let mut items = ncat.keys().map(|x| x.clone()).collect::<Vec<String>>();
							items.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

							println!("{}: {}", &category, items.join(", "));
						}
					}

					_ => match cmds.get(cat) {
						Some(category) => match category.get(&cmd) {
							Some(cmdfn) => match (cmdfn)(&mut context, args) {
								Ok(_) => {}
								Err(e) => {
									eprintln!("command error: {:?}", e);
								}
							},

							None => {
								eprintln!("invalid command: {}", [cat.clone(), cmd].join(" "));
							}
						},

						None => {
							eprintln!("invalid command: {}", [cat.clone(), cmd].join(" "));
						}
					},
				}
			}

			Err(ReadlineError::Interrupted) => {
				continue;
			}

			Err(ReadlineError::Eof) => {
				break;
			}

			Err(e) => {
				bail!("Unhandled ReadlineError: {:?}", e);
			}
		}
	}

	Ok(context)
}
