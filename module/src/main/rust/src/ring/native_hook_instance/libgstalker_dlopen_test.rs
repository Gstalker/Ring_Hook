use super::native_hook;
use super::native_hook::{
    NativeHookConfig,
    SymbolInfo
};
use std::mem::transmute;
use std::ffi::CStr;

#[inline(never)]
#[no_mangle]
pub extern "C" fn libgstalker_jnihook_test(
    env_sys: *mut jni::sys::JNIEnv,
    thiz: jni::sys::jobject,
    test_string: jni::sys::jstring
) -> usize{
    let backup_result = native_hook::get_backup_trampoline_by_hook_function_addr(
        libgstalker_jnihook_test as usize
    );
    let backup = match backup_result {
        None => {
            error!("cannot get backup!, program maybe crash soon later!");
            0 as usize
        }
        Some(backup_addr) => {
            backup_addr
        }
    };
    let backup: extern "C" fn(*mut jni::sys::JNIEnv, jni::sys::jobject, jni::sys::jstring) -> usize = unsafe{transmute(backup)};

    let env = unsafe{ jni::JNIEnv::from_raw(env_sys).unwrap() };
    let test_string_obj = unsafe{ jni::objects::JString::from(test_string) };
    let string_internal = env.get_string_utf_chars(test_string_obj).unwrap();
    warn!("libgstalker.so::native_test() detected: ");
    warn!("    teststring inner: {}", unsafe{ CStr::from_ptr(transmute(string_internal)).to_str().unwrap() });
    env.release_string_utf_chars(test_string_obj,string_internal);
    return backup(env_sys, thiz, test_string);
}

#[allow(unused_variables)]
pub fn register(env: &mut jni::JNIEnv,path :&String) {
    native_hook::register_native_hooker(
        NativeHookConfig::inline_config(
            libgstalker_jnihook_test as usize,
            None,
            Some(SymbolInfo::from_symbol_name_with_image_name(
                "Java_bin_gstalker_ring_test_Gstalker_target_1function".to_string(),
                "libgstalker.so".to_string())),
            true
        )
    );
}