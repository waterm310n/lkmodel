/// Todo: Extract proc_ops as standalone component.

use alloc::sync::Arc;
use alloc::string::String;
use alloc::format;
use axfs_vfs::VfsNodeOps;
use axfs_vfs::VfsNodeAttr;
use axfs_vfs::VfsNodePerm;
use axfs_vfs::VfsResult;
use axfs_vfs::VfsNodeType;
use axerrno::AxResult;
use axfile::fops::File;
use crate::OpenOptions;

struct ProcNode {
    path: String,
}

impl ProcNode {
    pub fn new(path: String) -> Self {
        Self {
            path,
        }
    }
}

impl VfsNodeOps for ProcNode {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        error!("VfsNode get_attr: {}", self.path);
        let perm = VfsNodePerm::from_bits_truncate(0o755);
        Ok(VfsNodeAttr::new(perm, VfsNodeType::File, 0, 0))
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        let offset: usize = offset as usize;
        match self.path.as_str() {
            "/proc/self/status" => {
                let mm = task::current().mm();
                let locked_mm = mm.lock();
                let src = format!("VmLck:         {} kB\n\0",
                    locked_mm.locked_vm << 2);
                buf[offset..src.len()].copy_from_slice(src.as_bytes());
                return Ok(buf.len());
            },
            _ => unimplemented!("openat path {}", self.path),
        }
    }
}

pub fn open(path: &str, opts: &OpenOptions) -> AxResult<File> {
    // Todo: handle self and [pid]
    let node = Arc::new(ProcNode::new(String::from(path)));
    Ok(File::new(node, opts.into()))
}
