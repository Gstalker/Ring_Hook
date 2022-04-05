use super::native_hook;
use std::ffi::CStr;
use std::mem::transmute;
use std::os::raw::c_char;
use jni::JNIEnv;
use crate::ring::native_hook::{InlineHookConfig, SymbolInfo};
use crate::ring::native_hook::NativeHookType::Inline;

#[inline(never)]
#[no_mangle]
pub extern "C" fn hook(level: usize, tag: *mut (), inner: *mut ()) -> usize {
    let backup_result = native_hook::Manager::from_instance()
        .lock()
        .unwrap()
        .get_backup_trampoline_by_hook_function_addr(hook as usize);
    let backup = match backup_result {
        None => {
            error!("cannot get backup!, program maybe crash soon later!");
            0 as usize
        }
        Some(backup_addr) => {
            backup_addr
        }
    };
    let backup: extern "C" fn(usize,*mut(),*mut()) -> usize = unsafe{transmute(backup)};

    let (tar_str,inner_str) = unsafe{
        (CStr::from_ptr(tag as *const c_char),CStr::from_ptr(inner as *const c_char))
    };
    warn!("target level:{} ,target TAG: {}, target info: {}",level,tar_str.to_str().unwrap(),inner_str.to_str().unwrap());
    return backup(level,tag,inner);
}

pub fn register(env: &mut JNIEnv,path :&String) {
    native_hook::Manager::from_instance().lock().unwrap().register_native_hooker(
        native_hook::NativeHookConfig::from(Inline(InlineHookConfig{
            hook: hook as usize,
            target: None,
            symbol_info: Some(SymbolInfo::from_symbol_name_with_image_name(
                "__android_log_print".to_string(),
                "liblog.so".to_string()
            )),
        }))
    );
}