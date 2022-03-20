use jni::JNIEnv;
use std::ffi::CStr;
use super::utils::enable_logger;

pub use crate::zygisk::{AppSpecializeArgs, ZygiskApi, ZygiskModule};
pub struct ZygiskEntry{}

#[allow(unused_variables)]
impl ZygiskModule for ZygiskEntry {
    fn on_load(&self, api: ZygiskApi, env: JNIEnv) {
        enable_logger();
        info!(target:"RING","Ring has been loaded!");
    }
    fn pre_app_specialize(&self, api: ZygiskApi, args: &mut AppSpecializeArgs) {
        info!(target:"RING","Hello world from pre-AppsecializeArgs");
    }
}
