#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_prelude)]
#![feature(trait_upcasting)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[macro_use]
extern crate slos_log;

#[allow(unused_imports)]
use self::alloc_prelude::*;
use alloc::prelude::v1 as alloc_prelude;

use core::fmt::{self, Debug};
use lasso::{Rodeo, Spur};
use slos_helpers::{StaticCollection, UnsafeContainer};

lazy_static::lazy_static! {
	static ref INTERNED: UnsafeContainer<Rodeo> = UnsafeContainer::new(Rodeo::new());
}

mod errors;
pub use self::errors::*;
pub mod memory;
pub mod path;

pub trait FsReadDir {
	fn readdir(&mut self) -> Result<Vec<&mut (dyn FsNode)>, FsError>;
}

pub trait FsWriteDir {
	fn touch(&mut self, name: &str) -> Result<&mut (dyn FsNode), FsError>;
}

pub trait FsRoot: Send + FsNode + FsReadDir + FsWriteDir + Debug {}

pub trait FsNode {
	fn mount(&self) -> Option<&dyn FsRoot>;
	fn inode(&self) -> usize;
	fn name(&self) -> &str;
	fn permissions(&self) -> u16;
	fn try_directory(&mut self) -> Option<&mut (dyn FsDirectory)>;
	fn try_file(&mut self) -> Option<&mut (dyn FsFile)>;
}

pub trait FsDirectory: FsNode + FsReadDir + FsWriteDir {}

pub trait FsFile: FsNode {
	fn open(&mut self) -> Result<&mut (dyn FsFileHandle), FsError>;
}

pub trait FsFileHandle {
	fn raw_read(&mut self, offset: usize, length: Option<usize>) -> Result<Vec<u8>, FsError>;
	fn raw_write(&mut self, offset: usize, data: &[u8]) -> Result<(), FsError>;
}

#[derive(Default)]
pub struct FilesystemMountpoint {
	pub path: StaticCollection<Option<Spur>>,
	pub root: Option<UnsafeContainer<Box<dyn FsRoot>>>,
}

impl FilesystemMountpoint {
	pub fn path_vec(&self) -> Vec<&'static str> {
		self.path
			.as_slice()
			.iter()
			.map(|x| x.unwrap_or(INTERNED.get().get_or_intern("[unknown]")))
			.map(|x| INTERNED.resolve(&x))
			.collect::<Vec<&str>>()
	}
}

impl Debug for FilesystemMountpoint {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("FilesystemMountpoint")
			.field("path", &self.path_vec())
			.finish()
	}
}

#[derive(Debug)]
pub struct FilesystemBase {
	pub mountpoints: StaticCollection<UnsafeContainer<FilesystemMountpoint>>,
}

impl FilesystemBase {
	pub fn new() -> Self {
		Self {
			mountpoints: StaticCollection::new(),
		}
	}

	pub fn mount(&mut self, path: &[&str], root: Box<dyn FsRoot>) -> Result<(), MountError> {
		let mut path_segments: StaticCollection<Option<Spur>> = StaticCollection::new();
		for seg in path.into_iter() {
			path_segments.push(Some(INTERNED.get().get_or_intern(seg)));
		}

		let mountpoint = FilesystemMountpoint {
			path: path_segments,
			root: Some(UnsafeContainer::new(root)),
		};

		trace!("mountpoint={:?}", &mountpoint);
		self.mountpoints.push(UnsafeContainer::new(mountpoint));
		Ok(())
	}

	pub fn node_at_path<'a>(&mut self, path: &[&str]) -> Result<&'a mut (dyn FsNode), FsError> {
		let path = crate::path::split(&crate::path::join(
			&path
				.iter()
				.map(|x| String::from(*x))
				.collect::<Vec<String>>(),
		));

		// find closest parent mountpoint
		let mut closest: Option<&UnsafeContainer<FilesystemMountpoint>> = None;
		if path.is_empty() {
			trace!("empty path, getting first empty root");
			'ep: for fs in self.mountpoints.as_slice().iter() {
				if fs.get().path_vec().is_empty() {
					trace!("fs={:?}", fs);
					closest = Some(fs);
					break 'ep;
				}
			}
		} else {
			let mut xsc = 0usize;
			for fs in self.mountpoints.as_slice().iter() {
				let pathvec = fs.get().path_vec();
				let mut startcount = 0usize;

				'uidx: for (unit, idx) in pathvec.iter().zip(0..) {
					if path.len() >= idx && &path[idx] == unit {
						startcount += 1;
					}

					break 'uidx;
				}

				if startcount > xsc {
					trace!("startcount={:?} xsc={:?} fs={:?}", startcount, xsc, fs);
					closest = Some(fs);
					xsc = startcount;
				}
			}
		}

		// if none closest, abort
		if closest.is_none() {
			trace!("couldn't find a mountpoint close to {:?}", &path);
			return Err(FsError::FileNotFound);
		}

		// unwrap the mountpoint
		let mountpoint = closest.unwrap();

		// get the remaining path segments after this mountpoint
		let path_remaining = {
			if mountpoint.get().path.as_slice().is_empty() {
				path
			} else {
				let (_, r) = path.split_at(mountpoint.get().path_vec().len());
				r.to_vec()
			}
		};

		trace!(
			"mountpoint={:?} path_remaining={:?}",
			mountpoint,
			path_remaining
		);

		// get the mountpoint as an FsNode
		let mount_root = match &mountpoint.get().root {
			Some(root) => root.get().as_mut() as &mut dyn FsNode,

			None => {
				return Err(FsError::FilesystemRootError);
			}
		};

		// reverse the list of path segments, to allow us to pop from this list
		let mut rev_segments = {
			let mut rev_segments = path_remaining.clone();
			rev_segments.reverse();
			rev_segments
		};

		// traverse the mountpoint for the node
		//
		// this will just fall through if there's nothing in `path_remaining`
		// so we'll automatically return the mountpoint root without having to
		// explicitly check for that
		let current_node: UnsafeContainer<&mut dyn FsNode> = UnsafeContainer::new(mount_root);
		'fsearch: while let Some(path_seg) = rev_segments.pop() {
			if let Some(dir) = current_node.get().try_directory() {
				for new in dir.readdir()? {
					if new.name() == path_seg {
						trace!("found next node, name={:?}", path_seg);
						current_node.replace(new);
						continue 'fsearch;
					}
				}
			}

			// if we have no path segments left at this point, we have our node
			if rev_segments.is_empty() {
				trace!("we've got our node, breaking 'fsearch");
				break 'fsearch;
			}

			// but if we've fallen through to here, and we have path segments left,
			// the path segment we need was not found in the current directory node,
			// so return a FileNotFound
			return Err(FsError::FileNotFound);
		}

		Ok(current_node.into_inner())
	}
}
