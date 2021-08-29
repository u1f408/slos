//! SimpleMemoryFilesystem - an in-memory single-directory read/write filesystem

use crate::alloc_prelude::*;
use crate::{FsDirectory, FsError, FsFile, FsFileHandle, FsNode, FsReadDir, FsRoot, FsWriteDir};

#[derive(Debug, PartialEq)]
pub struct SimpleMemoryFilesystem {
	pub files: Vec<SimpleMemoryFilesystemFile>,
	pub current_inode: usize,
}

impl SimpleMemoryFilesystem {
	pub fn new() -> Self {
		Self {
			files: Vec::new(),
			current_inode: 0,
		}
	}

	fn has_filename(&self, name: &str) -> bool {
		for f in self.files.iter() {
			if &f.name == name {
				return true;
			}
		}

		false
	}
}

impl FsReadDir for SimpleMemoryFilesystem {
	fn readdir(&mut self) -> Result<Vec<&mut dyn FsNode>, FsError> {
		let mut res = Vec::new();

		for file in self.files.iter_mut() {
			res.push(file as &mut dyn FsNode);
		}

		Ok(res)
	}
}

impl FsWriteDir for SimpleMemoryFilesystem {
	fn touch(&mut self, name: &str) -> Result<&mut dyn FsNode, FsError> {
		let name = String::from(name);
		if self.has_filename(&name) {
			return Err(FsError::FileExists);
		}

		self.current_inode += 1;

		let f = SimpleMemoryFilesystemFile {
			parent_index: None,
			inode: self.current_inode,
			name: name,
			content: Vec::new(),
		};

		self.files.push(f);
		Ok(self.files.last_mut().unwrap() as &mut dyn FsNode)
	}
}

impl FsNode for SimpleMemoryFilesystem {
	fn inode(&self) -> usize {
		0
	}

	fn name(&self) -> &str {
		""
	}

	fn permissions(&self) -> u16 {
		0o777
	}

	fn try_root(&mut self) -> Option<&mut dyn FsRoot> {
		Some(self as &mut dyn FsRoot)
	}

	fn try_directory(&mut self) -> Option<&mut dyn FsDirectory> {
		Some(self as &mut dyn FsDirectory)
	}

	fn try_file(&mut self) -> Option<&mut dyn FsFile> {
		None
	}
}

impl FsDirectory for SimpleMemoryFilesystem {}
impl FsRoot for SimpleMemoryFilesystem {}

#[derive(Debug, Default, PartialEq)]
pub struct SimpleMemoryFilesystemFile {
	pub parent_index: Option<usize>,
	pub inode: usize,
	pub name: String,
	pub content: Vec<u8>,
}

impl FsNode for SimpleMemoryFilesystemFile {
	fn inode(&self) -> usize {
		self.inode
	}

	fn name(&self) -> &str {
		&self.name
	}

	fn permissions(&self) -> u16 {
		0o777
	}

	fn try_root(&mut self) -> Option<&mut dyn FsRoot> {
		None
	}

	fn try_directory(&mut self) -> Option<&mut dyn FsDirectory> {
		None
	}

	fn try_file(&mut self) -> Option<&mut dyn FsFile> {
		Some(self as &mut dyn FsFile)
	}
}

impl FsFile for SimpleMemoryFilesystemFile {
	fn open(&mut self) -> Result<&mut dyn FsFileHandle, FsError> {
		Ok(self as &mut dyn FsFileHandle)
	}
}

impl FsFileHandle for SimpleMemoryFilesystemFile {
	fn raw_read(&mut self, offset: usize, length: Option<usize>) -> Result<Vec<u8>, FsError> {
		if offset > self.content.len() {
			return Err(FsError::EndOfFile);
		}

		let final_length = if length.is_none() {
			self.content.len() - offset
		} else {
			let length = length.unwrap();
			let mut final_length = self.content.len() - offset;
			if length < final_length {
				final_length = length;
			}
			final_length
		};

		return Ok(Vec::from(&self.content[offset..(offset + final_length)]));
	}

	fn raw_write(&mut self, offset: usize, data: &[u8]) -> Result<(), FsError> {
		if (offset + data.len()) > self.content.len() {
			for _ in self.content.len()..(offset + data.len()) {
				self.content.push(0);
			}
		}

		for (byte, idx) in data.iter().zip(offset..) {
			self.content[idx] = *byte;
		}

		Ok(())
	}
}
