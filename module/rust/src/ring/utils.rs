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
use nix::fcntl::{
    openat,OFlag
};
use nix::unistd::close;
use nix::sys::stat::Mode;

pub fn fd_test(fd: RawFd) {
    let ring = match openat(fd,"./dex/ring.dex",OFlag::O_RDONLY,Mode::empty()) {
        Ok(fd) => {
            info!("hello_world from ~/dex/ring.dex : {}", fd);
            fd
        },
        Err(_) => {
            error!("cannot open ring.dex!");
            return;
        }
    };
    close(ring);

}