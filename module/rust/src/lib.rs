#[macro_use]
extern crate log;
extern crate android_logger;

mod ring;
mod zygisk;

use jni::{
    JNIEnv
};
use std::os::unix::io::RawFd;
use ring::ZygiskEntry;

static MODULE: ZygiskEntry = ZygiskEntry {};

crate::zygisk_module!(&MODULE);

fn companion(_socket: RawFd) {}
crate::zygisk_companion!(companion);