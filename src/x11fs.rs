use fuse_mt::*;
use libc::ENOSYS;
use xcb;

use std::path::Path;

pub struct X11fs {
    conn: xcb::Connection,
}

impl X11fs {
    pub fn new(c: xcb::Connection) -> Self {
        X11fs {
            conn: c,
        }
    }
}

impl FilesystemMT for X11fs {
    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        Ok(())
    }

    fn getattr(&self, _req: RequestInfo, _path: &Path, _fh: Option<u64>) -> ResultEntry {
        Err(ENOSYS)
    }

    fn open(&self, _req: RequestInfo, _path: &Path, _flags: u32) -> ResultOpen {
        Err(ENOSYS)
    }

    fn read(&self, _req: RequestInfo, _path: &Path, _fh: u64, _offset: u64, _size: u32) -> ResultData {
        Err(ENOSYS)
    }

    fn write(&self, _req: RequestInfo, _path: &Path, _fh: u64, _offset: u64, _data: Vec<u8>, _flags: u32) -> ResultWrite {
        Err(ENOSYS)
    }

    fn readdir(&self, _req: RequestInfo, _path: &Path, _fh: u64) -> ResultReaddir {
        Err(ENOSYS)
    }
}
