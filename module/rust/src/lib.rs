#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate android_logger;

mod ring;
mod zygisk;

use std::os::unix::io::RawFd;
use ring::ZygiskEntry;

static MODULE: ZygiskEntry = ZygiskEntry {};

crate::zygisk_module!(&MODULE);

fn companion(_socket: RawFd) {}
crate::zygisk_companion!(companion);