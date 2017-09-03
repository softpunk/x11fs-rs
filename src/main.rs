extern crate fuse_mt;
extern crate xcb;
extern crate libc;
extern crate clap;

use clap::{App, Arg};

use std::process::exit;

mod x11fs;
use x11fs::X11fs;

fn main() {
    let (conn, _screen_num) = match xcb::Connection::connect(None) {
        Ok(c) => { c },
        Err(e) => {
            eprintln!("Could not connect to X server: {}", e);
            exit(1);
        },
    };

    let matches = App::new("x11fs-rs")
        .arg(Arg::with_name("mountpoint")
             .help("The mountpoint directory")
             .index(1)
             .required(true))
        .get_matches();

    let fs = X11fs::new(conn);
    let mountpoint = matches.value_of_os("mountpoint").unwrap();
    let fs_mt = fuse_mt::FuseMT::new(fs, 1);

    if let Err(e) = fuse_mt::mount(fs_mt, &mountpoint, &[]) {
        eprintln!("Failed to mount filesystem: {}", e);
        exit(1);
    }
}
