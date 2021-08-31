//! OverlayFS

use crate::alloc_prelude::*;
use crate::path as fspath;
use crate::{
	traverse_node, FsDirectory, FsError, FsFile, FsFileHandle, FsNode, FsReadDir, FsRoot,
	FsWriteDir,
};

use alloc::sync::Arc;
use core::cmp::PartialEq;
use core::fmt::{self, Debug};
use slos_helpers::UnsafeContainer;

static mut CURRENT_INODE: usize = 0xC31A0000;

#[derive(Debug, PartialEq, Copy, Clone)]
enum NodeType {
	Unknown,
	Directory,
	File,
}

impl From<&mut dyn FsNode> for NodeType {
	fn from(node: &mut dyn FsNode) -> NodeType {
		if node.try_file().is_some() {
			NodeType::File
		} else if node.try_directory().is_some() {
			NodeType::Directory
		} else {
			NodeType::Unknown
		}
	}
}

/// An overlay filesystem proxy.
pub struct OverlayFilesystem<'a, 'base, 'overlay> {
	pub inner: Arc<OverlayFilesystemInner<'a, 'base, 'overlay>>,
	base_path: OverlayFilesystemPath<'a, 'base, 'overlay>,
}

impl<'a, 'base, 'overlay> OverlayFilesystem<'a, 'base, 'overlay> {
	pub fn new(
		name: &'a str,
		base: &'base mut dyn FsRoot,
		overlay: &'overlay mut dyn FsRoot,
	) -> Self {
		let inner = Arc::new(OverlayFilesystemInner {
			base: Arc::new(base),
			overlay: Arc::new(overlay),
			name: name,
			inode: unsafe {
				CURRENT_INODE += 1;
				CURRENT_INODE
			},
		});

		unsafe { Self::new_from_inner(inner) }
	}

	pub unsafe fn new_from_inner(inner: Arc<OverlayFilesystemInner<'a, 'base, 'overlay>>) -> Self {
		let base_path = OverlayFilesystemPath::new_from_inner(Arc::clone(&inner));
		Self { inner, base_path }
	}
}

impl<'a, 'base, 'overlay> Debug for OverlayFilesystem<'a, 'base, 'overlay> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("OverlayFilesystem")
			.field("inner", &self.inner)
			.field("base_path", &self.base_path)
			.finish()
	}
}

impl<'a, 'base, 'overlay> PartialEq for OverlayFilesystem<'a, 'base, 'overlay> {
	fn eq(&self, other: &OverlayFilesystem<'a, 'base, 'overlay>) -> bool {
		self.inner.as_ref().eq(other.inner.as_ref())
	}
}

impl<'a, 'base, 'overlay> FsReadDir for OverlayFilesystem<'a, 'base, 'overlay> {
	fn readdir(&mut self) -> Result<Vec<&mut dyn FsNode>, FsError> {
		self.base_path.readdir()
	}
}

impl<'a, 'base, 'overlay> FsWriteDir for OverlayFilesystem<'a, 'base, 'overlay> {
	fn touch(&mut self, name: &str) -> Result<&mut dyn FsNode, FsError> {
		self.base_path.touch(name)
	}
}

impl<'a, 'base, 'overlay> FsNode for OverlayFilesystem<'a, 'base, 'overlay> {
	fn inode(&self) -> usize {
		self.base_path.inode()
	}

	fn name(&self) -> &str {
		self.inner.name
	}

	fn permissions(&self) -> u16 {
		0o777
	}

	fn try_root(&mut self) -> Option<&mut dyn FsRoot> {
		Some(self as &mut dyn FsRoot)
	}

	fn try_directory(&mut self) -> Option<&mut dyn FsDirectory> {
		self.base_path.try_directory()
	}

	fn try_file(&mut self) -> Option<&mut dyn FsFile> {
		None
	}
}

impl<'a, 'base, 'overlay> FsDirectory for OverlayFilesystem<'a, 'base, 'overlay> {}
impl<'a, 'base, 'overlay> FsRoot for OverlayFilesystem<'a, 'base, 'overlay> {}

#[derive(Clone)]
pub struct OverlayFilesystemPath<'a, 'base, 'overlay> {
	pub inner: Arc<OverlayFilesystemInner<'a, 'base, 'overlay>>,
	pub path: Vec<String>,
	node_type: NodeType,
	children: Vec<Self>,
}

impl<'a, 'base, 'overlay> OverlayFilesystemPath<'a, 'base, 'overlay> {
	pub fn new_from_inner(inner: Arc<OverlayFilesystemInner<'a, 'base, 'overlay>>) -> Self {
		Self {
			inner: Arc::clone(&inner),
			path: Vec::new(),
			node_type: NodeType::Directory,
			children: Vec::new(),
		}
	}

	fn update_children(&mut self) {
		self.children.clear();

		let mut children: Vec<(String, NodeType)> = Vec::new();
		let inner = unsafe { Arc::get_mut_unchecked(&mut self.inner) };

		{
			let overlay = unsafe { Arc::get_mut_unchecked(&mut inner.overlay) };
			if let Ok(rd) = overlay.readdir() {
				for node in rd {
					let name = String::from(node.name());
					let node_type = NodeType::from(node);
					trace!("overlay child: name={:?} node_type={:?}", &name, &node_type);
					children.push((name, node_type));
				}
			}
		}

		{
			let base = unsafe { Arc::get_mut_unchecked(&mut inner.base) };
			if let Ok(rd) = base.readdir() {
				for node in rd {
					let name = String::from(node.name());
					if !children.iter().find(|x| x.0 == name).is_some() {
						let node_type = NodeType::from(node);
						trace!("base child: name={:?} node_type={:?}", &name, &node_type);
						children.push((name, node_type));
					}
				}
			}
		}

		for (name, node_type) in children.iter() {
			let mut path = self.path.clone();
			path.push(name.to_string());

			let node = Self {
				inner: Arc::clone(&self.inner),
				path: fspath::split(&fspath::join(&path)),
				node_type: *node_type,
				children: Vec::new(),
			};

			self.children.push(node);
		}
	}

	/// Is this node a file?
	pub fn is_file(&self) -> bool {
		self.node_type == NodeType::File
	}

	/// Is this node a directory?
	pub fn is_directory(&self) -> bool {
		self.node_type == NodeType::Directory
	}

	/// Try to get the inner node for the current path.
	///
	/// If `ignore_root` is `true`, this ignores filesystem roots when finding
	/// a return value. If `ignore_root` is `false`, and we have an empty path,
	/// this will return the filesystem root for the overlay.
	pub fn try_get_inner_node(&mut self, ignore_root: bool) -> Option<&mut dyn FsNode> {
		let inner = unsafe { Arc::get_mut_unchecked(&mut self.inner) };
		let base = unsafe { Arc::get_mut_unchecked(&mut inner.base) };
		let overlay = unsafe { Arc::get_mut_unchecked(&mut inner.overlay) };

		match traverse_node(*overlay, self.path.clone(), ignore_root) {
			Some(node) => Some(node),
			None => match traverse_node(*base, self.path.clone(), ignore_root) {
				Some(node) => Some(node),
				None => None,
			},
		}
	}
}

impl<'a, 'base, 'overlay> Debug for OverlayFilesystemPath<'a, 'base, 'overlay> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("OverlayFilesystemPath")
			.field("inner", &self.inner)
			.field("path", &self.path)
			.field("children", &self.children)
			.finish()
	}
}

impl<'a, 'base, 'overlay> PartialEq for OverlayFilesystemPath<'a, 'base, 'overlay> {
	fn eq(&self, other: &OverlayFilesystemPath<'a, 'base, 'overlay>) -> bool {
		self.inner.as_ref().eq(other.inner.as_ref()) && self.path.eq(&other.path)
	}
}

impl<'a, 'base, 'overlay> FsReadDir for OverlayFilesystemPath<'a, 'base, 'overlay> {
	fn readdir(&mut self) -> Result<Vec<&mut dyn FsNode>, FsError> {
		let mut nodes = Vec::new();

		self.update_children();
		for node in self.children.iter_mut() {
			nodes.push(node as &mut dyn FsNode);
		}

		Ok(nodes)
	}
}

impl<'a, 'base, 'overlay> FsWriteDir for OverlayFilesystemPath<'a, 'base, 'overlay> {
	fn touch(&mut self, name: &str) -> Result<&mut dyn FsNode, FsError> {
		let inner = self
			.try_get_inner_node(false)
			.ok_or(FsError::FileNotFound)?;

		let directory = inner.try_directory().ok_or(FsError::FileNotFound)?;
		directory.touch(name)
	}

	fn mkdir(&mut self, name: &str) -> Result<&mut dyn FsNode, FsError> {
		let inner = self
			.try_get_inner_node(false)
			.ok_or(FsError::FileNotFound)?;

		let directory = inner.try_directory().ok_or(FsError::FileNotFound)?;
		directory.mkdir(name)
	}
}

impl<'a, 'base, 'overlay> FsNode for OverlayFilesystemPath<'a, 'base, 'overlay> {
	fn inode(&self) -> usize {
		0
	}

	fn name(&self) -> &str {
		if self.path.len() > 0 {
			&self.path[self.path.len() - 1]
		} else {
			""
		}
	}

	fn permissions(&self) -> u16 {
		0o777
	}

	fn try_root(&mut self) -> Option<&mut dyn FsRoot> {
		None
	}

	fn try_directory(&mut self) -> Option<&mut dyn FsDirectory> {
		if self.is_directory() {
			Some(self as &mut dyn FsDirectory)
		} else {
			None
		}
	}

	fn try_file(&mut self) -> Option<&mut dyn FsFile> {
		if self.is_file() {
			Some(self as &mut dyn FsFile)
		} else {
			None
		}
	}
}

impl<'a, 'base, 'overlay> FsFile for OverlayFilesystemPath<'a, 'base, 'overlay> {
	fn open(&mut self) -> Result<&mut dyn FsFileHandle, FsError> {
		Ok(self as &mut dyn FsFileHandle)
	}
}

impl<'a, 'base, 'overlay> FsFileHandle for OverlayFilesystemPath<'a, 'base, 'overlay> {
	fn raw_read(&mut self, offset: usize, length: Option<usize>) -> Result<Vec<u8>, FsError> {
		let inner = self.try_get_inner_node(true).ok_or(FsError::FileNotFound)?;
		let file = inner.try_file().ok_or(FsError::FileNotFound)?;
		let handle = file.open().or(Err(FsError::FileNotFound))?;

		handle.raw_read(offset, length)
	}

	fn raw_write(&mut self, offset: usize, data: &[u8]) -> Result<(), FsError> {
		// Get our filesystem roots
		let inner = unsafe { Arc::get_mut_unchecked(&mut self.inner) };
		let base = unsafe { Arc::get_mut_unchecked(&mut inner.base) };
		let overlay = unsafe { Arc::get_mut_unchecked(&mut inner.overlay) };

		let handle = if let Some(overlaynode) = traverse_node(*overlay, self.path.clone(), true) {
			// Found the node on the overlay, use the handle to that
			let file = overlaynode.try_file().ok_or(FsError::FileNotFound)?;
			let handle = file.open().or(Err(FsError::FileNotFound))?;
			handle
		} else {
			// Couldn't find the node on the overlay, so:
			//
			// - recursively `mkdir` the base path on the overlay
			// - `touch` the file on the overlay
			// - copy the contents of the same path on the base to the overlay
			// - use the handle to the new file on the overlay

			let current_node: UnsafeContainer<&mut dyn FsNode> = UnsafeContainer::new(*overlay);

			let mut base_path = self.path.clone();
			let name = base_path.pop().ok_or(FsError::Unknown)?;

			// recursive mkdir
			for path_seg in base_path {
				if let Some(dir) = current_node.get().try_directory() {
					current_node.replace(dir.mkdir(&path_seg)?);
				} else {
					trace!("mkdir: current_node.try_directory() failed");
					return Err(FsError::FileNotFound);
				}
			}

			// touch
			if let Some(dir) = current_node.get().try_directory() {
				current_node.replace(dir.touch(&name)?);
			} else {
				trace!("touch: current_node.try_directory() failed");
				return Err(FsError::FileNotFound);
			}

			// current_node is our new file on the overlay
			let overlaynode = current_node.into_inner();
			let overlayfile = overlaynode.try_file().ok_or(FsError::FileNotFound)?;
			let overlayhandle = overlayfile.open().or(Err(FsError::FileNotFound))?;

			// copy file content from base
			if let Some(basenode) = traverse_node(*base, self.path.clone(), true) {
				let basefile = basenode.try_file().ok_or(FsError::FileNotFound)?;
				let basehandle = basefile.open().or(Err(FsError::FileNotFound))?;

				overlayhandle.raw_write(0, &basehandle.raw_read(0, None)?)?;
			}

			overlayhandle
		};

		handle.raw_write(offset, data)
	}
}

impl<'a, 'base, 'overlay> FsDirectory for OverlayFilesystemPath<'a, 'base, 'overlay> {}

#[derive(Clone)]
pub struct OverlayFilesystemInner<'a, 'base, 'overlay> {
	pub name: &'a str,
	pub inode: usize,
	pub base: Arc<&'base mut dyn FsRoot>,
	pub overlay: Arc<&'overlay mut dyn FsRoot>,
}

impl<'a, 'base, 'overlay> Debug for OverlayFilesystemInner<'a, 'base, 'overlay> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("OverlayFilesystemInner")
			.field("name", &self.name)
			.field("inode", &self.inode)
			.field("base", &self.base)
			.field("overlay", &self.overlay)
			.finish()
	}
}

impl<'a, 'base, 'overlay> PartialEq for OverlayFilesystemInner<'a, 'base, 'overlay> {
	fn eq(&self, other: &OverlayFilesystemInner<'a, 'base, 'overlay>) -> bool {
		if self.base.name() == other.base.name()
			&& self.base.inode() == other.base.inode()
			&& self.base.permissions() == other.base.permissions()
		{
			if self.overlay.name() == other.overlay.name()
				&& self.overlay.inode() == other.overlay.inode()
				&& self.overlay.permissions() == other.overlay.permissions()
			{
				return true;
			}
		}

		false
	}
}

unsafe impl<'a, 'base, 'overlay> Send for OverlayFilesystemInner<'a, 'base, 'overlay> {}
unsafe impl<'a, 'base, 'overlay> Sync for OverlayFilesystemInner<'a, 'base, 'overlay> {}
