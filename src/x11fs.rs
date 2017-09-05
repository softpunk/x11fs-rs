use fuse_mt::*;
use libc::{ENOSYS, ENOENT};
use xcb;

use std::path::Path;
use std::ffi::OsString;

pub struct X11fs {
    conn: xcb::Connection,
}

impl X11fs {
    pub fn new(c: xcb::Connection) -> Self {
        X11fs {
            conn: c,
        }
    }

    pub fn list_windows(&self) -> Vec<String> {
        // Iterators are cool
        // Maybe I can get this to work later

        // self.conn.get_setup().roots().flat_map(|screen| {
        //     xcb::query_tree(&self.conn, screen.root()).get_reply().iter().flat_map(|tree| {
        //         tree.children().iter().map(|window| {
        //             format!("0x{:08x}", window)
        //         })
        //     })
        // }).collect::<Vec<String>>()

        let mut windows = Vec::new();
        let setup = self.conn.get_setup();
        for screen in setup.roots() {
            match xcb::query_tree(&self.conn, screen.root()).get_reply() {
                Ok(tree) => {
                    for window in tree.children() {
                        windows.push(format!("0x{:08x}", window));
                    }
                },
                Err(_) => {
                    continue;
                },
            }
        }
        windows
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
