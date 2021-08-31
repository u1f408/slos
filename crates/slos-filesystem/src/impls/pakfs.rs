//! PAK-archive-as-filesystem implementation

use crate::alloc_prelude::*;
use crate::path as fspath;
use crate::{FsDirectory, FsError, FsFile, FsFileHandle, FsNode, FsReadDir, FsRoot, FsWriteDir};

use alloc::sync::Arc;
use core::cmp::PartialEq;
use core::fmt::{self, Debug};

use rpak::PakArchive;

static mut CURRENT_INODE: usize = 0xC3010000;

pub struct PakFilesystem<'a> {
	pub inner: Arc<PakFilesystemInner<'a>>,
	base_path: PakFilesystemPath<'a>,
}

impl<'a> PakFilesystem<'a> {
	pub fn from_bytes(name: &'static str, data: &'a [u8]) -> Result<Self, ()> {
		let inner = Arc::new(PakFilesystemInner {
			archive: Arc::new(PakArchive::from_bytes(data)?),
			name,
			inode: unsafe {
				CURRENT_INODE += 1;
				CURRENT_INODE
			},
		});

		Ok(unsafe { Self::new_from_inner(inner) })
	}

	pub unsafe fn new_from_inner(inner: Arc<PakFilesystemInner<'a>>) -> Self {
		let base_path = PakFilesystemPath::new_from_inner(Arc::clone(&inner));
		Self { inner, base_path }
	}
}

impl<'a> Debug for PakFilesystem<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PakFilesystem")
			.field("inner", &self.inner.as_ref())
			.field("base_path", &self.base_path)
			.finish()
	}
}

impl<'a> FsReadDir for PakFilesystem<'a> {
	fn readdir(&mut self) -> Result<Vec<&mut dyn FsNode>, FsError> {
		self.base_path.readdir()
	}
}

impl<'a> FsWriteDir for PakFilesystem<'a> {
	fn touch(&mut self, _name: &str) -> Result<&mut dyn FsNode, FsError> {
		Err(FsError::ReadOnlyFilesystem)
	}

	fn mkdir(&mut self, _name: &str) -> Result<&mut dyn FsNode, FsError> {
		Err(FsError::ReadOnlyFilesystem)
	}
}

impl<'a> FsNode for PakFilesystem<'a> {
	fn inode(&self) -> usize {
		self.inner.inode
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

	fn try_directory(&mut self) -> Option<&mut (dyn FsDirectory)> {
		Some(&mut self.base_path as &mut dyn FsDirectory)
	}

	fn try_file(&mut self) -> Option<&mut (dyn FsFile)> {
		None
	}
}

impl<'a> FsDirectory for PakFilesystem<'a> {}
impl<'a> FsRoot for PakFilesystem<'a> {}

pub struct PakFilesystemInner<'a> {
	pub archive: Arc<PakArchive<'a>>,
	pub name: &'a str,
	pub inode: usize,
}

impl<'a> Debug for PakFilesystemInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PakFilesystemInner")
			.field("name", &self.name)
			.field("inode", &self.inode)
			.finish()
	}
}

impl<'a> PartialEq for PakFilesystemInner<'a> {
	fn eq(&self, other: &PakFilesystemInner<'_>) -> bool {
		if self.name != other.name {
			return false;
		}

		self.archive.files == other.archive.files
	}
}

#[derive(Clone)]
pub struct PakFilesystemPath<'a> {
	parent: Arc<PakFilesystemInner<'a>>,
	pub children: Vec<Self>,
	pub index: Option<usize>,
	pub path: Vec<String>,
}

impl<'a> PakFilesystemPath<'a> {
	pub fn new_from_inner(inner: Arc<PakFilesystemInner<'a>>) -> Self {
		Self {
			parent: Arc::clone(&inner),
			children: Vec::new(),
			index: None,
			path: Vec::new(),
		}
	}

	fn file_data(&self) -> Option<&'a [u8]> {
		match self.index {
			Some(index) => Some(self.parent.as_ref().archive.files[index].1),
			None => None,
		}
	}

	fn update_children(&mut self) {
		let parent = unsafe { Arc::get_mut_unchecked(&mut self.parent) };
		let mut children = Vec::new();

		for (index, file) in (0..).zip(parent.archive.files.iter()) {
			let file_path = fspath::split(&fspath::normalize(&file.0));
			if let Some(subslice) = file_path.strip_prefix(self.path.clone().as_slice()) {
				if subslice.len() == 1 {
					children.push((Some(index), file_path));
				} else {
					let dirpath = if file_path.len() == 1 {
						file_path.clone()
					} else {
						Vec::from(&file_path[..(file_path.len() + 1) - subslice.len()])
					};

					children.push((None, dirpath))
				}
			}
		}

		self.children.clear();
		for (index, path) in children {
			self.children.push(Self {
				parent: Arc::clone(&self.parent),
				children: Vec::new(),
				index,
				path,
			});
		}
	}
}

impl<'a> Debug for PakFilesystemPath<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PakFilesystemPath")
			.field("path", &self.path)
			.field("index", &self.index)
			.field("children", &self.children)
			.finish()
	}
}

impl<'a> PartialEq for PakFilesystemPath<'a> {
	fn eq(&self, other: &PakFilesystemPath<'_>) -> bool {
		if self.path != other.path || self.index != other.index {
			return false;
		}

		self.parent == other.parent
	}
}

impl<'a> FsNode for PakFilesystemPath<'a> {
	fn inode(&self) -> usize {
		self.index.unwrap_or(0)
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
		match self.index {
			Some(_) => None,
			None => {
				if self.path.len() == 0 {
					None
				} else {
					Some(self as &mut dyn FsDirectory)
				}
			}
		}
	}

	fn try_file(&mut self) -> Option<&mut dyn FsFile> {
		match self.index {
			Some(_) => Some(self as &mut dyn FsFile),
			None => None,
		}
	}
}

impl<'a> FsReadDir for PakFilesystemPath<'a> {
	fn readdir(&mut self) -> Result<Vec<&mut dyn FsNode>, FsError> {
		self.update_children();

		let mut res = Vec::new();
		for child in self.children.iter_mut() {
			res.push(child as &mut dyn FsNode);
		}

		Ok(res)
	}
}

impl<'a> FsWriteDir for PakFilesystemPath<'a> {
	fn touch(&mut self, _name: &str) -> Result<&mut dyn FsNode, FsError> {
		Err(FsError::ReadOnlyFilesystem)
	}

	fn mkdir(&mut self, _name: &str) -> Result<&mut dyn FsNode, FsError> {
		Err(FsError::ReadOnlyFilesystem)
	}
}

impl<'a> FsDirectory for PakFilesystemPath<'a> {}

impl<'a> FsFile for PakFilesystemPath<'a> {
	fn open(&mut self) -> Result<&mut dyn FsFileHandle, FsError> {
		Ok(self as &mut dyn FsFileHandle)
	}
}

impl<'a> FsFileHandle for PakFilesystemPath<'a> {
	fn raw_read(&mut self, offset: usize, length: Option<usize>) -> Result<Vec<u8>, FsError> {
		if let Some(content) = self.file_data() {
			if offset > content.len() {
				return Err(FsError::EndOfFile);
			}

			let final_length = if length.is_none() {
				content.len() - offset
			} else {
				let length = length.unwrap();
				let mut final_length = content.len() - offset;
				if length < final_length {
					final_length = length;
				}
				final_length
			};

			return Ok(Vec::from(&content[offset..(offset + final_length)]));
		} else {
			Err(FsError::InvalidArgument)
		}
	}

	fn raw_write(&mut self, _offset: usize, _data: &[u8]) -> Result<(), FsError> {
		Err(FsError::ReadOnlyFilesystem)
	}
}
