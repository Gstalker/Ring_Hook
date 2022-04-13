use super::utils::{read_dex_file,enable_logger};
use super::init_helper::INIT_HELPER;
use crate::zygisk::{AppSpecializeArgs, ZygiskApi, ZygiskModule,ZygiskOption};
use super::manager::RingManager;

use std::ffi::CStr;
use std::sync::Mutex;
use std::os::unix::io::RawFd;
use jni::JNIEnv;
use crate::ring::manager::Config;


pub struct ZygiskEntry{}

#[allow(unused_variables)]
impl ZygiskModule for ZygiskEntry {
    fn on_load(&self, api: ZygiskApi, env: *mut ()) {
        enable_logger();
        INIT_HELPER.lock().unwrap().update_env(env);
        let flag = api.get_flags();
    }

    fn pre_app_specialize(&self, api: ZygiskApi, args: &mut AppSpecializeArgs) {
        let env = unsafe {
            JNIEnv::from_raw(INIT_HELPER.lock().unwrap().env().unwrap().cast()).unwrap()
        };

        let nice_name_c_str = match env.get_string_utf_chars(*args.nice_name) {
            Ok(ptr) => {ptr}
            Err(e) => {
                error!("app_name_c_str is nullptr! skip this Application");
                return;
            }
        };
        let app_name = unsafe{CStr::from_ptr(nice_name_c_str.cast()).to_str().unwrap().to_string()};
        let mut app_name_guard = APPLICATION_NAME.lock().unwrap();
        *app_name_guard = app_name.clone();
        env.release_string_utf_chars(*args.nice_name,nice_name_c_str).unwrap();
        trace!("app_name: {}", app_name);

        // read dex_file
        let dir_fd = api.get_module_dir();
        let dex_file = match read_dex_file(dir_fd as RawFd) {
            Ok(dex_file) => {dex_file}
            Err(e) => {
                error!("cannot load ring.dex!");
                return;
            }
        };
        DEX_FILES_DATA.lock().unwrap().push(dex_file);
    }

    fn post_app_specialize(&self, api: ZygiskApi, args: &AppSpecializeArgs) {
        let mut manager_guard = RingManager::from_instance().lock().unwrap();
        let manager = &mut *manager_guard;
        let mut app_name_guard = APPLICATION_NAME.lock().unwrap();
        let app_name = &mut *app_name_guard;
        let mut env = unsafe {
            JNIEnv::from_raw(INIT_HELPER.lock().unwrap().env().unwrap().cast()).unwrap()
        };

        let (native_config,dalvik_config,root_path) = match INIT_HELPER.lock().unwrap().read_config(app_name.clone()) {
            None => {return;}
            Some(config) => {config}
        };

        manager.setup(
            Config::default(root_path)
                .enable_dalvik_hook(dalvik_config)
                .enable_native_hook(native_config)
                .with_dex_files_data(DEX_FILES_DATA.lock().unwrap().clone())
        );

        let unload = manager.process(&mut env);

        if unload == false {
            api.set_option(ZygiskOption::DlcloseModuleLibrary);
        }
        INIT_HELPER.lock().unwrap().deactivate();
    }
}

lazy_static!{
    static ref APPLICATION_NAME : Mutex<String> = Mutex::new(String::new());
    static ref DEX_FILES_DATA: Mutex<Vec<Vec<u8>>> = Mutex::new(Vec::new());
}