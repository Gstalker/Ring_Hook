use super::utils::enable_logger;
use crate::zygisk::{AppSpecializeArgs, ZygiskApi, ZygiskModule};
use super::init_helper::INIT_HELPER;

pub struct ZygiskEntry{}

#[allow(unused_variables)]
impl ZygiskModule for ZygiskEntry {
    fn on_load(&self, api: ZygiskApi, env: *mut ()) {
        enable_logger();
        INIT_HELPER.lock().unwrap().update_env(env);
        let flag = api.get_flags();
        info!(target:"RING","Ring has been loaded!");
        let dir = api.get_module_dir();
        info!(target:"RING","module_dir‘s fd : {}",dir);
    }
    fn pre_app_specialize(&self, api: ZygiskApi, args: &mut AppSpecializeArgs) {
        info!(target:"RING","Hello world from pre-AppsecializeArgs");
    }
    fn post_app_specialize(&self, api: ZygiskApi, args: &AppSpecializeArgs) {
        INIT_HELPER.lock().unwrap().deactivate();
    }
}
