extern crate fuse_mt;
use fuse_mt::FilesystemMT;

extern crate xcb;

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
}
