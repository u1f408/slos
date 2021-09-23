//! Filesystem command set

use anyhow::{bail, Context as AnyhowContext, Result};

use super::Context;
use slos_filesystem::impls::SimpleMemoryFilesystem;

/// Print a normalized path
pub fn cmd_path_normalize(_context: &mut Context, args: &[String]) -> Result<()> {
	for path in args.iter() {
		let npath = slos_filesystem::path::normalize(path);
		println!("{}", npath);
	}

	Ok(())
}

/// Debug print a split path
pub fn cmd_path_split(_context: &mut Context, args: &[String]) -> Result<()> {
	for path in args.iter() {
		let npath = slos_filesystem::path::split(path);
		println!("{:?}", npath);
	}

	Ok(())
}

/// Print a joined path
pub fn cmd_path_join(_context: &mut Context, args: &[String]) -> Result<()> {
	let npath = slos_filesystem::path::join(args);
	println!("{}", npath);

	Ok(())
}

/// Print the list of currently mounted filesystems
pub fn cmd_mount_list(context: &mut Context, _args: &[String]) -> Result<()> {
	for mp in context.fs.mountpoints.as_slice().iter() {
		let pathvec = mp
			.path_vec()
			.iter()
			.map(|x| String::from(*x))
			.collect::<Vec<String>>();
		println!("{} - {:?}", slos_filesystem::path::join(&pathvec), mp.root);
	}

	Ok(())
}

/// Mount a new [`SimpleMemoryFilesystem`] on the given path
pub fn cmd_mount_new_memoryfs(context: &mut Context, args: &[String]) -> Result<()> {
	let path = if let Some(path) = args.first() {
		slos_filesystem::path::split(path)
	} else {
		bail!("invalid path")
	};

	let path_split = path.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

	context
		.fs
		.mount(&path_split, Box::new(SimpleMemoryFilesystem::new()))
		.with_context(|| "failed to mount new SimpleMemoryFilesystem")?;

	Ok(())
}

/// Read the contents of the given file
pub fn cmd_file_read(context: &mut Context, args: &[String]) -> Result<()> {
	let path = if let Some(path) = args.first() {
		slos_filesystem::path::split(path)
	} else {
		bail!("invalid path")
	};

	let path_split = path.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

	let node = context
		.fs
		.node_at_path(&path_split)
		.context("couldn't get node")?;
	let filenode = node.try_file().context("failed to FsNode.try_file")?;
	let filehandle = filenode.open().context("failed to open file")?;
	let content = filehandle
		.raw_read(0, None)
		.context("failed to read file")?;

	println!("[bytes] {:?}", content);
	if let Ok(s) = String::from_utf8(content.to_vec()) {
		println!("[string] {:?}", s);
	}

	Ok(())
}

/// Write the string `"hello world!"` to the given file
pub fn cmd_file_write_test(context: &mut Context, args: &[String]) -> Result<()> {
	let path = if let Some(path) = args.first() {
		slos_filesystem::path::split(path)
	} else {
		bail!("invalid path")
	};

	let mut path_split = path.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

	let filename = if let Some(filename) = path_split.pop() {
		filename
	} else {
		bail!("invalid path")
	};

	let parent = context
		.fs
		.node_at_path(&path_split)
		.context("couldn't get parent")?;
	let parentdir = parent
		.try_directory()
		.context("parent is not a directory")?;
	let filenode = parentdir.touch(filename).context("failed to create file")?;
	let filenode = filenode.try_file().context("failed to FsNode.try_file")?;
	let filehandle = filenode.open().context("failed to open file")?;
	filehandle
		.raw_write(0, b"hello world!")
		.context("failed to write to file")?;

	Ok(())
}
