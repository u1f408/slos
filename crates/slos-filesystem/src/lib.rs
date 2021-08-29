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
	/// String interner used for path segments
	static ref INTERNED: UnsafeContainer<Rodeo> = UnsafeContainer::new(Rodeo::new());
}

mod errors;
pub use self::errors::*;
pub mod memory;
pub mod path;

/// Directory read functions
pub trait FsReadDir {
	/// Return an [`FsNode`] reference for each node in this directory
	fn readdir(&mut self) -> Result<Vec<&mut (dyn FsNode)>, FsError>;
}

/// Directory write functions
pub trait FsWriteDir {
	/// Create a new empty file in this directory
	fn touch(&mut self, name: &str) -> Result<&mut (dyn FsNode), FsError>;
}

/// Mountable filesystem root
pub trait FsRoot: Send + FsNode + FsReadDir + FsWriteDir + Debug {}

/// Filesystem node
pub trait FsNode: Debug {
	/// Try to get the root filesystem this node belongs to
	fn mount(&self) -> Option<&dyn FsRoot>;

	/// Get the inode value for this node
	fn inode(&self) -> usize;

	/// Get the filename of this node
	fn name(&self) -> &str;

	/// Get the permissions of this node
	fn permissions(&self) -> u16;

	/// Try to get this node as a [`FsDirectory`] trait object reference
	///
	/// Will always return [`None`][Option::None] if this node is a file.
	fn try_directory(&mut self) -> Option<&mut (dyn FsDirectory)>;

	/// Try to get this node as a [`FsFile`] trait object reference
	///
	/// Will always return [`None`][Option::None] if this node is a directory.
	fn try_file(&mut self) -> Option<&mut (dyn FsFile)>;
}

/// A directory on a filesystem
pub trait FsDirectory: FsNode + FsReadDir + FsWriteDir {}

/// A file on a filesystem
pub trait FsFile: FsNode {
	fn open(&mut self) -> Result<&mut (dyn FsFileHandle), FsError>;
}

/// Read/write handle to a [`FsFile`]
pub trait FsFileHandle {
	/// Try to read from the file
	///
	/// Attempts to read `length` bytes from the `offset` into the file.
	/// If `length` is [`None`][Option::None], read from `offset` to the
	/// end of the file.
	///
	/// If the file is write-only, this should return [`FsError::InvalidArgument`].
	///
	/// If the file is only able to be read from as a stream (for example,
	/// character devices such as terminals), in the case where `offset` is
	/// non-zero and/or `length` is not [`None`][Option::None], this should
	/// return [`FsError::InvalidArgument`].
	fn raw_read(&mut self, offset: usize, length: Option<usize>) -> Result<Vec<u8>, FsError>;

	/// Try to write to the file
	///
	/// Attempts to write the `data` to the `offset` into the file, replacing
	/// any existing content.
	///
	/// It is implementation-dependent whether this function will truncate the
	/// supplied data if it is too large to fit in the file (for example, block
	/// devices), however implementations **should** try to allocate more space
	/// on the physical filesystem and extend the file to fit the entire data.
	///
	/// If the file is read-only, this should return [`FsError::InvalidArgument`].
	///
	/// If the file is only able to be written to as a stream (for example,
	/// character devices such as terminals), in the case where `offset` is
	/// non-zero, this should return [`FsError::InvalidArgument`].
	fn raw_write(&mut self, offset: usize, data: &[u8]) -> Result<(), FsError>;
}

/// Container for filesystem mountpoint roots
///
/// This mostly exists as an implementation detail of [`FilesystemBase`].
#[derive(Default)]
pub struct FilesystemMountpoint {
	/// Segments of the path making up the mountpoint
	///
	/// Paths are stored internally as interned strings, see the
	/// [`path_string`][FilesystemMountpoint::path_string] method to get the
	/// path in a format that can be used outside of this module.
	pub path: StaticCollection<Option<Spur>>,

	/// Filesystem root, as a contained [`FsRoot`] trait object
	pub root: Option<UnsafeContainer<Box<dyn FsRoot>>>,
}

impl FilesystemMountpoint {
	/// Get the absolute path to this mountpoint
	///
	/// This function takes care of resolving the interned strings that make
	/// up the `path` field of this structure, but no post-conversion path
	/// normalization is performed before returning.
	///
	/// If you're planning to use the path for anything other than passing straight
	/// back into [`FilesystemBase`], unless you _really_ know what you're doing,
	/// please use the [`path_string`][FilesystemMountpoint::path_string] method
	/// instead.
	pub fn path_vec(&self) -> Vec<&'static str> {
		self.path
			.as_slice()
			.iter()
			.map(|x| x.unwrap_or(INTERNED.get().get_or_intern("[unknown]")))
			.map(|x| INTERNED.resolve(&x))
			.collect::<Vec<&str>>()
	}

	/// Get the absolute path to this mountpoint as a String
	///
	/// This function performs path normalization, so the returned path is safe
	/// to use in non-internal methods.
	pub fn path_string(&self) -> String {
		let segs = self
			.path_vec()
			.iter()
			.map(|x| String::from(*x))
			.collect::<Vec<String>>();

		path::join(&segs)
	}
}

impl Debug for FilesystemMountpoint {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("FilesystemMountpoint")
			.field("path", &self.path_vec())
			.finish()
	}
}

/// Base structure for mounting filesystems to
#[derive(Debug)]
pub struct FilesystemBase {
	/// Collection of mounted filesystems
	pub mountpoints: StaticCollection<UnsafeContainer<FilesystemMountpoint>>,
}

impl FilesystemBase {
	/// Create a new empty `FilesystemBase` instance.
	pub fn new() -> Self {
		Self {
			mountpoints: StaticCollection::new(),
		}
	}

	/// Mount a filesystem root at `path`.
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

	/// Return an [`FsNode`] for the given `path`, if one exists.
	///
	/// If the given `path` is exactly the root of a mounted filesystem, returns
	/// the filesystem root object, casted to an [`FsNode`] (so you can treat it
	/// as a normal directory).
	///
	/// If the given `path` is not the root of a mounted filesystem, traverses
	/// the parent filesystem of the path to find the node at the path within
	/// that filesystem.
	///
	/// Returns [`FsError::FileNotFound`] if a node could not be found at the path,
	/// or [`FsError::FilesystemRootError`] if the parent filesystem of the path
	/// was not able to be cast to an [`FsNode`] (which would only ever happen if
	/// mounting the filesystem failed spectactularly, or the mountpoint table had
	/// been messed with).
	pub fn node_at_path<'a>(&mut self, path: &[&str]) -> Result<&'a mut (dyn FsNode), FsError> {
		let path = crate::path::split(&crate::path::join(
			&path
				.iter()
				.map(|x| String::from(*x))
				.collect::<Vec<String>>(),
		));

		// find closest parent mountpoint …
		let mut closest: Option<&UnsafeContainer<FilesystemMountpoint>> = None;

		// … starting with the root filesystem …
		'ep: for fs in self.mountpoints.as_slice().iter() {
			if fs.get().path_vec().is_empty() {
				trace!("fs={:?}", fs);
				closest = Some(fs);
				break 'ep;
			}
		}

		// … and then checking for path prefixes
		let mut xsc = 0usize;
		for fs in self.mountpoints.as_slice().iter() {
			let pathvec = fs.get().path_vec();
			let mut startcount = 0usize;

			'uidx: for (unit, idx) in pathvec.iter().zip(0..) {
				if path.len() >= idx && &path[idx] == unit {
					startcount += 1;
				} else {
					break 'uidx;
				}
			}

			if startcount > xsc {
				trace!("startcount={:?} xsc={:?} fs={:?}", startcount, xsc, fs);
				closest = Some(fs);
				xsc = startcount;
			}
		}

		// if none closest (which would only happen if we have no rootfs) then
		// return a FileNotFound
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
