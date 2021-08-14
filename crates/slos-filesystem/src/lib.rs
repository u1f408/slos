#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_prelude)]
#![feature(trait_upcasting)]

#[cfg(feature = "std")]
extern crate std;
extern crate alloc;

use alloc::prelude::v1 as alloc_prelude;
#[allow(unused_imports)]
use self::alloc_prelude::*;

use core::fmt::{self, Debug};
use slos_helpers::{StaticCollection, UnsafeContainer};
use lasso::{Rodeo, Spur};

lazy_static::lazy_static! {
    static ref INTERNED: UnsafeContainer<Rodeo> = UnsafeContainer::new(Rodeo::new());
}

mod errors;
pub use self::errors::*;
pub mod path;
pub mod memory;

pub trait FsReadDir {
    fn readdir(&mut self) -> Result<Vec<&mut (dyn FsNode)>, FsError>;
}

pub trait FsWriteDir {
    fn touch(&mut self, name: &str) -> Result<&mut (dyn FsNode), FsError>;
}

pub trait FsRoot: FsNode + FsReadDir + FsWriteDir + Debug { }

pub trait FsNode {
    fn mount(&self) -> Option<&dyn FsRoot>;
    fn inode(&self) -> usize;
    fn name(&self) -> &str;
    fn permissions(&self) -> u16;
    fn try_directory(&mut self) -> Option<&mut (dyn FsDirectory)>;
    fn try_file(&mut self) -> Option<&mut (dyn FsFile)>;
}

pub trait FsDirectory: FsNode + FsReadDir + FsWriteDir { }

pub trait FsFile: FsNode {
    fn open(&mut self) -> Result<&mut (dyn FsFileHandle), FsError>;
}

pub trait FsFileHandle {
    fn file(&mut self) -> &mut (dyn FsFile);
    fn read(&mut self, offset: usize, length: Option<usize>) -> Result<&[u8], FsError>;
    fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), FsError>;
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

        self.mountpoints.push(UnsafeContainer::new(mountpoint));
        Ok(())
    }

    pub fn node_at_path(&mut self, path: &[&str]) -> Result<&mut (dyn FsNode), FsError> {
        let path = crate::path::split(&crate::path::join(
            &path.iter().map(|x| String::from(*x)).collect::<Vec<String>>()
        ));

        #[cfg(feature = "std")]
        std::dbg!(&path);

        // find closest parent mountpoint
        let mut closest: Option<&UnsafeContainer<FilesystemMountpoint>> = None;
        if path.is_empty() {
            for fs in self.mountpoints.as_slice().iter() {
                if fs.get().path_vec().is_empty() {
                    closest = Some(fs);
                }
            }

        } else {
            let xsc = 0usize;
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
                    closest = Some(fs);
                }
            }
        }

        // if none closest, abort
        if closest.is_none() {
            return Err(FsError::FileNotFound);
        }

        // unwrap the mountpoint
        let mountpoint = closest.unwrap();
        #[cfg(feature = "std")]
        std::dbg!(&mountpoint);

        // get the remaining path segments after this mountpoint
        let path_remaining = {
            if mountpoint.get().path.as_slice().is_empty() {
                path
            } else {
                let (_, r) = path.split_at(mountpoint.get().path_vec().len());
                r.to_vec()
            }
        };
        #[cfg(feature = "std")]
        std::dbg!(&path_remaining);

        // if remaining path is empty, return the mountpoint itself
        if path_remaining.is_empty() {
            return match &mountpoint.get().root {
                Some(root) => Ok(root.get().as_mut() as &mut dyn FsNode),
                None => Err(FsError::FilesystemRootError),
            }
        }

        // traverse that mountpoint for the node
        Err(FsError::Unknown)
    }
}
