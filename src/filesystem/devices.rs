use crate::alloc_prelude::*;
use core::fmt::{self, Debug};

use crate::current_system;
use slos_filesystem::{
	FsDirectory, FsError, FsFile, FsFileHandle, FsNode, FsReadDir, FsRoot, FsWriteDir,
};
use slos_helpers::{StaticCollection, UnsafeContainer};

pub fn system_devices() -> SystemDeviceCollection {
	let mut collection = SystemDeviceCollection::new();

	// System console
	collection.push(SystemDeviceFile {
		name: "console",
		inode: 0xF3F30001,
		device: UnsafeContainer::new(current_system().console()),
	});

	collection
}

#[derive(Debug)]
pub struct SystemDeviceCollection {
	pub devices: StaticCollection<Option<SystemDeviceFile>>,
}

impl SystemDeviceCollection {
	pub fn new() -> Self {
		Self {
			devices: StaticCollection::new(),
		}
	}

	pub fn push(&mut self, file: SystemDeviceFile) {
		self.devices.push(Some(file));
	}
}

impl FsReadDir for SystemDeviceCollection {
	fn readdir(&mut self) -> Result<Vec<&mut (dyn FsNode)>, FsError> {
		let mut res = Vec::new();

		for file in self.devices.as_mut_slice().iter_mut() {
			if let Some(file) = file {
				res.push(file as &mut dyn FsNode);
			}
		}

		Ok(res)
	}
}

impl FsWriteDir for SystemDeviceCollection {
	fn touch(&mut self, _name: &str) -> Result<&mut (dyn FsNode), FsError> {
		Err(FsError::InvalidArgument)
	}
}

impl FsNode for SystemDeviceCollection {
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

impl FsDirectory for SystemDeviceCollection {}
impl FsRoot for SystemDeviceCollection {}

unsafe impl Send for SystemDeviceCollection {}
unsafe impl Sync for SystemDeviceCollection {}

pub struct SystemDeviceFile {
	pub name: &'static str,
	pub inode: usize,
	device: UnsafeContainer<&'static mut dyn FsFileHandle>,
}

impl FsNode for SystemDeviceFile {
	fn mount(&self) -> Option<&dyn FsRoot> {
		None
	}

	fn inode(&self) -> usize {
		self.inode
	}

	fn name(&self) -> &str {
		self.name
	}

	fn permissions(&self) -> u16 {
		0o666
	}

	fn try_directory(&mut self) -> Option<&mut (dyn FsDirectory)> {
		None
	}

	fn try_file(&mut self) -> Option<&mut (dyn FsFile)> {
		Some(self as &mut dyn FsFile)
	}
}

impl FsFile for SystemDeviceFile {
	fn open(&mut self) -> Result<&mut (dyn FsFileHandle), FsError> {
		Ok(*self.device.get())
	}
}

impl Debug for SystemDeviceFile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("SystemDeviceFile")
			.field("name", &self.name)
			.field("inode", &self.inode)
			.finish()
	}
}
