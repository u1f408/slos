use crate::alloc_prelude::*;
use crate::{FsDirectory, FsError, FsFile, FsFileHandle, FsNode, FsReadDir, FsRoot, FsWriteDir};

#[derive(Debug, PartialEq)]
pub struct SimpleMemoryFs {
	pub files: Vec<SimpleMemoryFsFile>,
	pub current_inode: usize,
}

impl SimpleMemoryFs {
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

impl FsReadDir for SimpleMemoryFs {
	fn readdir(&mut self) -> Result<Vec<&mut (dyn FsNode)>, FsError> {
		let mut res = Vec::new();

		for file in self.files.iter_mut() {
			res.push(file as &mut dyn FsNode);
		}

		Ok(res)
	}
}

impl FsWriteDir for SimpleMemoryFs {
	fn touch(&mut self, name: &str) -> Result<&mut (dyn FsNode), FsError> {
		let name = String::from(name);
		if self.has_filename(&name) {
			return Err(FsError::FileExists);
		}

		self.current_inode += 1;

		let f = SimpleMemoryFsFile {
			parent_index: None,
			inode: self.current_inode,
			name: name,
			content: Vec::new(),
		};

		self.files.push(f);
		Ok(self.files.last_mut().unwrap() as &mut dyn FsNode)
	}
}

impl FsNode for SimpleMemoryFs {
	fn mount(&self) -> Option<&dyn FsRoot> {
		Some(self as &dyn FsRoot)
	}

	fn inode(&self) -> usize {
		0
	}

	fn name(&self) -> &str {
		""
	}

	fn permissions(&self) -> u16 {
		0o777
	}

	fn try_directory(&mut self) -> Option<&mut (dyn FsDirectory)> {
		Some(self as &mut dyn FsDirectory)
	}

	fn try_file(&mut self) -> Option<&mut (dyn FsFile)> {
		None
	}
}

impl FsDirectory for SimpleMemoryFs {}
impl FsRoot for SimpleMemoryFs {}

#[derive(Debug, Default, PartialEq)]
pub struct SimpleMemoryFsFile {
	pub parent_index: Option<usize>,
	pub inode: usize,
	pub name: String,
	pub content: Vec<u8>,
}

impl FsNode for SimpleMemoryFsFile {
	fn mount(&self) -> Option<&dyn FsRoot> {
		None
	}

	fn inode(&self) -> usize {
		self.inode
	}

	fn name(&self) -> &str {
		&self.name
	}

	fn permissions(&self) -> u16 {
		0o777
	}

	fn try_directory(&mut self) -> Option<&mut (dyn FsDirectory)> {
		None
	}

	fn try_file(&mut self) -> Option<&mut (dyn FsFile)> {
		Some(self as &mut dyn FsFile)
	}
}

impl FsFile for SimpleMemoryFsFile {
	fn open(&mut self) -> Result<&mut (dyn FsFileHandle), FsError> {
		Ok(self as &mut dyn FsFileHandle)
	}
}

impl FsFileHandle for SimpleMemoryFsFile {
	fn file(&mut self) -> &mut (dyn FsFile) {
		self as &mut dyn FsFile
	}

	fn read(&mut self, offset: usize, length: Option<usize>) -> Result<&[u8], FsError> {
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

		return Ok(&self.content[offset..(offset + final_length)]);
	}

	fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), FsError> {
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
