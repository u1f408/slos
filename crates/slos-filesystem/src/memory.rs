use crate::alloc_prelude::*;
use crate::{FsError, FsRoot, FsReadDir, FsWriteDir, FsNode, FsDirectory, FsFile, FsFileHandle};

#[derive(Debug)]
pub struct MemoryFs<'a> {
    pub root_files: Vec<MemoryFsFile<'a>>,
    pub root_directories: Vec<MemoryFsDirectory<'a>>,
    current_inode: usize,
}

impl MemoryFs<'_> {
    pub fn new() -> Self {
        Self {
            root_files: Vec::new(),
            root_directories: Vec::new(),
            current_inode: 0,
        }
    }

    fn has_root_filename(&self, name: &String) -> bool {
        for f in self.root_files.iter() {
            if &f.name == name {
                return true;
            }
        }

        // for f in self.root_directories.iter() {
        //     if f.name == name {
        //         return true;
        //     }
        // }

        false
    }
}

impl FsReadDir for MemoryFs<'_> {
    fn readdir(&mut self) -> Result<Vec<&mut (dyn FsNode)>, FsError> {
        let mut res = Vec::new();

        for file in self.root_files.iter_mut() {
            res.push(file as &mut dyn FsNode);
        }

        // for dir in self.root_directories.iter_mut() {
        //     res.push(dir as &mut dyn FsNode);
        // }

        Ok(res)
    }
}

impl<'a> FsWriteDir for MemoryFs<'a> {
    fn touch(&mut self, name: &str) -> Result<&mut (dyn FsNode), FsError> {
        let name = String::from(name);
        if self.has_root_filename(&name) {
            return Err(FsError::FileExists);
        }

        self.current_inode += 1;

        let f = MemoryFsFile {
            parent: None, // TODO: figure this shit out
            inode: self.current_inode,
            name: name,
            content: Vec::new(),
        };

        self.root_files.push(f);
        Ok(self.root_files.last_mut().unwrap() as &mut dyn FsNode)
    }
}

impl FsNode for MemoryFs<'_> {
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

impl FsDirectory for MemoryFs<'_> { }
impl FsRoot for MemoryFs<'_> { }

#[derive(Debug, Default)]
pub struct MemoryFsFile<'a> {
    pub parent: Option<&'a MemoryFs<'a>>,
    pub inode: usize,
    pub name: String,
    pub content: Vec<u8>,
}

impl FsNode for MemoryFsFile<'_> {
    fn mount(&self) -> Option<&dyn FsRoot> {
        if let Some(p) = self.parent {
            Some(p as &dyn FsRoot)
        } else {
            None
        }
    }

    fn inode(&self) -> usize {        self.inode
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

impl FsFile for MemoryFsFile<'_> {
    fn open(&mut self) -> Result<&mut (dyn FsFileHandle), FsError> {
        Ok(self as &mut dyn FsFileHandle)
    }
}

impl FsFileHandle for MemoryFsFile<'_> {
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

        return Ok(&self.content[offset..(offset + final_length)])
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

#[derive(Debug)]
pub struct MemoryFsDirectory<'a> {
    pub path: Vec<String>,
    pub files: Vec<MemoryFsFile<'a>>,
}
