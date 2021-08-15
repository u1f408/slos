use anyhow::{bail, Result, Context as AnyhowContext};

use super::Context;
use slos_filesystem::memory::SimpleMemoryFs;

pub fn cmd_path_normalize(_context: &mut Context, args: &[String]) -> Result<()> {
    for path in args.iter() {
        let npath = slos_filesystem::path::normalize(path);
        println!("{}", npath);
    }

    Ok(())
}

pub fn cmd_path_split(_context: &mut Context, args: &[String]) -> Result<()> {
    for path in args.iter() {
        let npath = slos_filesystem::path::split(path);
        println!("{:?}", npath);
    }

    Ok(())
}

pub fn cmd_path_join(_context: &mut Context, args: &[String]) -> Result<()> {
    let npath = slos_filesystem::path::join(args);
    println!("{}", npath);

    Ok(())
}

pub fn cmd_mount_list(context: &mut Context, _args: &[String]) -> Result<()> {
    for mp in context.fs.mountpoints.as_slice().iter() {
        let pathvec = mp.path_vec().iter().map(|x| String::from(*x)).collect::<Vec<String>>();
        println!("{} - {:?}", slos_filesystem::path::join(&pathvec), mp.root);
    }

    Ok(())
}

pub fn cmd_mount_new_memoryfs(context: &mut Context, args: &[String]) -> Result<()> {
    let path = if let Some(path) = args.first() {
        slos_filesystem::path::split(path)
    } else {
        bail!("invalid path")
    };

    let path_split = path
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();

    context.fs.mount(&path_split, Box::new(SimpleMemoryFs::new()))
        .with_context(|| "failed to mount new MemoryFs")?;

    Ok(())
}

pub fn cmd_file_read(context: &mut Context, args: &[String]) -> Result<()> {
    let path = if let Some(path) = args.first() {
        slos_filesystem::path::split(path)
    } else {
        bail!("invalid path")
    };

    let mut path_split = path
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();

    let filename = if let Some(filename) = path_split.pop() {
        filename
    } else {
        bail!("invalid path")
    };

    let parent = context.fs.node_at_path(&path_split).context("couldn't get parent")?;
    let parentdir = parent.try_directory().context("parent is not a directory")?;
    let mut readdir = parentdir.readdir().context("couldn't read directory")?;
    let node = readdir
        .iter_mut()
        .filter(|x| x.name() == filename)
        .next()
        .context("no node with that name")?;
    let filenode = node.try_file().context("failed to FsNode.try_file")?;
    let filehandle = filenode.open().context("failed to open file")?;
    let content = filehandle.read(0, None).context("failed to read file")?;

    println!("[bytes] {:?}", content);
    if let Ok(s) = String::from_utf8(content.to_vec()) {
        println!("[string] {:?}", s);
    }

    Ok(())
}

pub fn cmd_file_write_test(context: &mut Context, args: &[String]) -> Result<()> {
    let path = if let Some(path) = args.first() {
        slos_filesystem::path::split(path)
    } else {
        bail!("invalid path")
    };

    let mut path_split = path
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();

    let filename = if let Some(filename) = path_split.pop() {
        filename
    } else {
        bail!("invalid path")
    };

    let parent = context.fs.node_at_path(&path_split).context("couldn't get parent")?;
    let parentdir = parent.try_directory().context("parent is not a directory")?;
    let filenode = parentdir.touch(filename).context("failed to create file")?;
    let filenode = filenode.try_file().context("failed to FsNode.try_file")?;
    let filehandle = filenode.open().context("failed to open file")?;
    filehandle.write(0, b"hello world!").context("failed to write to file")?;

    Ok(())
}