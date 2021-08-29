//! OverlayFS

use crate::alloc_prelude::*;
use crate::path as fspath;
use crate::{FsDirectory, FsError, FsFile, FsFileHandle, FsNode, FsReadDir, FsRoot, FsWriteDir};

use alloc::sync::Arc;
use core::cmp::PartialEq;
use core::fmt::{self, Debug};

static mut CURRENT_INODE: usize = 0xC31A0000;

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
		f.debug_tuple("OverlayFilesystem")
			.field(&self.inner)
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
	children: Vec<Self>,
}

impl<'a, 'base, 'overlay> OverlayFilesystemPath<'a, 'base, 'overlay> {
	pub fn new_from_inner(inner: Arc<OverlayFilesystemInner<'a, 'base, 'overlay>>) -> Self {
		Self {
			inner: Arc::clone(&inner),
			path: Vec::new(),
			children: Vec::new(),
		}
	}

	pub fn update_children(&mut self) {
		let names = {
			let mut names = Vec::new();

			if let Some(inner) = Arc::get_mut(&mut self.inner) {
				if let Some(overlay) = Arc::get_mut(&mut inner.overlay) {
					if let Ok(rd) = overlay.readdir() {
						for node in rd.iter() {
							names.push(String::from(node.name()));
						}
					}
				}

				if let Some(base) = Arc::get_mut(&mut inner.base) {
					if let Ok(rd) = base.readdir() {
						for node in rd.iter() {
							names.push(String::from(node.name()));
						}
					}
				}
			}

			names
		};

		self.children.clear();
		for name in names.iter() {
			let mut path = self.path.clone();
			path.push(name.to_string());

			let node = Self {
				inner: Arc::clone(&self.inner),
				path: fspath::split(&fspath::join(&path)),
				children: Vec::new(),
			};

			self.children.push(node);
		}
	}

	pub fn is_file(&self) -> bool {
		false
	}
}

impl<'a, 'base, 'overlay> Debug for OverlayFilesystemPath<'a, 'base, 'overlay> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("OverlayFilesystemPath")
			.field("inner", &self.inner)
			.field("path", &self.path)
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
		self.update_children();
		let mut nodes = Vec::new();

		for node in self.children.iter_mut() {
			nodes.push(node as &mut dyn FsNode);
		}

		Ok(nodes)
	}
}

impl<'a, 'base, 'overlay> FsWriteDir for OverlayFilesystemPath<'a, 'base, 'overlay> {
	fn touch(&mut self, _name: &str) -> Result<&mut dyn FsNode, FsError> {
		Err(FsError::InvalidArgument)
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
		if self.is_file() {
			None
		} else {
			Some(self as &mut dyn FsDirectory)
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
		Err(FsError::InvalidArgument)
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
