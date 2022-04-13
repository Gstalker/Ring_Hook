use android_logger::Config;
use std::sync::Mutex;
use log::Level;

lazy_static!{
    pub static ref ENABLE_NATIVE_HOOK_CONFIG_FILE : Mutex<String> = Mutex::new(String::from("/enable_native"));
    pub static ref ENABLE_DALVIK_HOOK_CONFIG_FILE : Mutex<String> = Mutex::new(String::from("/enable_dalvik"));
}

#[inline(always)]
pub fn enable_logger() {
    android_logger::init_once(
        Config::default().with_min_level(Level::Trace).with_tag("RING"));
}



use std::os::unix::io::RawFd;
use std::fs::File;
use std::io::Read;
use std::os::unix::prelude::FromRawFd;
use nix::fcntl::{
    openat,OFlag
};
use nix::sys::stat::Mode;
use anyhow::{Result};

pub fn read_dex_file(fd: RawFd) -> Result<Vec<u8>>{
    let ring = match openat(fd,"./dex/ring.dex",OFlag::O_RDONLY,Mode::empty()) {
        Ok(fd) => {
            info!("hello_world from ~/dex/ring.dex : {}", fd);
            fd
        },
        Err(e) => {
            error!("cannot open ring.dex!");
            return Err(e.into());
        }
    };
    let mut file = unsafe{ File::from_raw_fd(ring)};
    let mut inner: Vec<u8> = Vec::new();
    match file.read_to_end(&mut inner) {
        Ok(_) => {
            Ok(inner)
        }
        Err(e) => {
            Err(e.into())
        }
    }
}