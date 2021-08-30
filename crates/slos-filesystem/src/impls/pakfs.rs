//! PAK-archive-as-filesystem implementation

use crate::alloc_prelude::*;
use crate::path as fspath;
use crate::{FsDirectory, FsError, FsFile, FsFileHandle, FsNode, FsReadDir, FsRoot, FsWriteDir};

use alloc::sync::Arc;
use core::cmp::PartialEq;
use core::fmt::{self, Debug};

use rpak::PakArchive;

pub struct PakFilesystem<'a> {
	pub archive: Arc<PakArchive<'a>>,
	pub files: Vec<PakFilesystemFile<'a>>,
	pub name: &'a str,
}

impl<'a> PakFilesystem<'a> {
	pub fn from_bytes(name: &'static str, data: &'a [u8]) -> Result<Self, ()> {
		let mut s = Self {
			archive: Arc::new(PakArchive::from_bytes(data)?),
			files: Vec::new(),
			name,
		};

		s.populate_files();
		Ok(s)
	}

	fn populate_files(&mut self) {
		self.files.clear();
		for (index, (name, _)) in (0..).zip(self.archive.as_ref().files.iter()) {
			let path = fspath::split(&name);
			self.files.push(PakFilesystemFile {
				parent: Arc::clone(&self.archive),
				index: Some(index),
				path,
			});
		}
	}
}

impl<'a> Debug for PakFilesystem<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PakFilesystem")
			.field("name", &self.name)
			.field("files", &self.files)
			.finish()
	}
}

impl<'a> PartialEq for PakFilesystem<'a> {
	fn eq(&self, other: &PakFilesystem<'_>) -> bool {
		if self.name != other.name {
			return false;
		}

		self.archive.files == other.archive.files
	}
}

impl<'a> FsReadDir for PakFilesystem<'a> {
	fn readdir(&mut self) -> Result<Vec<&mut dyn FsNode>, FsError> {
		let mut res = Vec::new();
		for file in self.files.iter_mut() {
			res.push(file as &mut dyn FsNode);
		}

		Ok(res)
	}
}

impl<'a> FsWriteDir for PakFilesystem<'a> {
	fn touch(&mut self, _name: &str) -> Result<&mut dyn FsNode, FsError> {
		Err(FsError::ReadOnlyFilesystem)
	}
}

impl<'a> FsNode for PakFilesystem<'a> {
	fn inode(&self) -> usize {
		0
	}

	fn name(&self) -> &str {
		self.name
	}

	fn permissions(&self) -> u16 {
		0o777
	}

	fn try_root(&mut self) -> Option<&mut dyn FsRoot> {
		Some(self as &mut dyn FsRoot)
	}

	fn try_directory(&mut self) -> Option<&mut (dyn FsDirectory)> {
		Some(self as &mut dyn FsDirectory)
	}

	fn try_file(&mut self) -> Option<&mut (dyn FsFile)> {
		None
	}
}

impl<'a> FsDirectory for PakFilesystem<'a> {}
impl<'a> FsRoot for PakFilesystem<'a> {}

#[derive(Clone)]
pub struct PakFilesystemFile<'a> {
	parent: Arc<PakArchive<'a>>,
	pub index: Option<usize>,
	pub path: Vec<String>,
}

impl<'a> PakFilesystemFile<'a> {
	fn file_data(&self) -> Option<&'a [u8]> {
		match self.index {
			Some(index) => Some(self.parent.as_ref().files[index].1),
			None => None,
		}
	}
}

impl<'a> Debug for PakFilesystemFile<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PakFilesystemFile")
			.field("path", &self.path)
			.field("index", &self.index)
			.finish()
	}
}

impl<'a> PartialEq for PakFilesystemFile<'a> {
	fn eq(&self, other: &PakFilesystemFile<'_>) -> bool {
		if self.path != other.path || self.index != other.index {
			return false;
		}

		self.parent.files == other.parent.files
	}
}

impl<'a> FsNode for PakFilesystemFile<'a> {
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

impl<'a> FsReadDir for PakFilesystemFile<'a> {
	fn readdir(&mut self) -> Result<Vec<&mut dyn FsNode>, FsError> {
		let res = Vec::new();

		// TODO: this

		Ok(res)
	}
}

impl<'a> FsWriteDir for PakFilesystemFile<'a> {
	fn touch(&mut self, _name: &str) -> Result<&mut dyn FsNode, FsError> {
		Err(FsError::ReadOnlyFilesystem)
	}
}

impl<'a> FsDirectory for PakFilesystemFile<'a> {}

impl<'a> FsFile for PakFilesystemFile<'a> {
	fn open(&mut self) -> Result<&mut dyn FsFileHandle, FsError> {
		Ok(self as &mut dyn FsFileHandle)
	}
}

impl<'a> FsFileHandle for PakFilesystemFile<'a> {
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
