use android_logger::Config;
use log::{Level,error};

pub fn enable_logger() {
    android_logger::init_once(
        Config::default().with_min_level(Level::Trace));
    trace!(target:"RING","this is a verbose {}", "message");
    error!(target:"RING","this is printed by default");
}