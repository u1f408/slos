//! Core filesystem implementations

pub mod overlayfs;
pub use self::overlayfs::OverlayFilesystem;

pub mod pakfs;
pub use self::pakfs::PakFilesystem;

pub mod simplememoryfs;
pub use self::simplememoryfs::SimpleMemoryFilesystem;
