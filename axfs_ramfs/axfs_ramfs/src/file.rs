use alloc::vec::Vec;
use axfs_vfs::{impl_vfs_non_dir_default, VfsNodeAttr, VfsNodeOps, VfsResult};
use spin::RwLock;

/// The file node in the RAM filesystem.
///
/// It implements [`axfs_vfs::VfsNodeOps`].
pub struct FileNode {
    content: RwLock<Vec<u8>>,
}

impl FileNode {
    pub(super) const fn new() -> Self {
        Self {
            content: RwLock::new(Vec::new()),
        }
    }
}

impl VfsNodeOps for FileNode {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        Ok(VfsNodeAttr::new_file(self.content.read().len() as _, 0))
    }

    fn truncate(&self, size: u64) -> VfsResult {
        let mut content = self.content.write();
        if size < content.len() as u64 {
            content.truncate(size as _);
        } else {
            content.resize(size as _, 0);
        }
        Ok(())
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        let content = self.content.read();
        let start = content.len().min(offset as usize);
        let end = content.len().min(offset as usize + buf.len());
        let src = &content[start..end];
        buf[..src.len()].copy_from_slice(src);
        Ok(src.len())
    }

    fn write_at(&self, offset: u64, buf: &[u8]) -> VfsResult<usize> {
        let offset = offset as usize;
        let mut content = self.content.write();
        if offset + buf.len() > content.len() {
            content.resize(offset + buf.len(), 0);
        }
        let dst = &mut content[offset..offset + buf.len()];
        dst.copy_from_slice(&buf[..dst.len()]);
        Ok(buf.len())
    }

    impl_vfs_non_dir_default! {}
}

/// The Fifo file node in the RAM filesystem.
///
/// It implements [`axfs_vfs::VfsNodeOps`].
pub struct FifoNode {
    content: RwLock<Vec<u8>>, // use 2 st to pseudo implement VecDeque
}

impl FifoNode {
    pub(super) const fn new() -> Self {
        Self {
            content: RwLock::new(Vec::new()),
        }
    }
}

impl VfsNodeOps for FifoNode {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        Ok(VfsNodeAttr::new_fifo(
            self.content.read().len() as _,
            0,
        ))
    }

    fn read_at(&self, _offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        let mut content = self.content.write();
        if content.len() == 0  {
            return Err(axfs_vfs::VfsError::WouldBlock);
        }
        let size = buf.len().min(content.len());
        let content: Vec<u8> = content.drain(0..size).collect();
        buf[..size].copy_from_slice(&content);
        Ok(size)
    }

    fn write_at(&self, _offset: u64, buf: &[u8]) -> VfsResult<usize> {
        let mut content = self.content.write();
        content.extend_from_slice(buf);
        Ok(buf.len())
    }

    impl_vfs_non_dir_default! {}
}
