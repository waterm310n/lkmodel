#[cfg(feature = "ext2fs")]
pub mod ext2fs;

#[cfg(feature = "fatfs")]
pub mod fatfs;

#[cfg(feature = "devfs")]
pub use axfs_devfs as devfs;

#[cfg(feature = "ramfs")]
pub use axfs_ramfs as ramfs;

mod path;
