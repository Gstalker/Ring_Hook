use super::native_hook;
use super::native_hook::{
    NativeHookConfig,
    SymbolInfo
};
use std::mem::transmute;
use std::ffi::CStr;
use std::sync::Mutex;
use jni::JNIEnv;

#[inline(never)]
#[no_mangle]
pub extern "C" fn dlopen_hooker(
    dylib_name: *mut (),
    flag: usize,
    android_ext_info: *mut (),
    caller_address: usize
) -> usize{
    let backup_result = native_hook::get_backup_trampoline_by_hook_function_addr(
        dlopen_hooker as usize
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
    let backup: extern "C" fn(*mut (), usize, *mut (), usize) -> usize = unsafe{transmute(backup)};
    trace!("dylib loading detected: ");
    trace!("    libname: {}",unsafe{ CStr::from_ptr(transmute(dylib_name)).to_str().unwrap() });
    trace!("    flag:    {:o}", flag);
    trace!("    caller:0x{:x}", caller_address);
    let result =  backup(dylib_name, flag, android_ext_info,caller_address);
    trace!("    ret value(handle): {:x}", result);

    // process hookers foro classloader-namespace(/data/app/xxx/lib)
    native_hook::process_hookers();

    return result;
}

#[allow(unused_variables)]
pub fn register(env: &mut JNIEnv,path :&String) {
    native_hook::register_native_hooker(
        NativeHookConfig::inline_config(
            dlopen_hooker as usize,
            None,
            Some(SymbolInfo::from_symbol_name_with_image_name(
                "__dl__Z9do_dlopenPKciPK17android_dlextinfoPKv".to_string(),
                "linker".to_string())),
            true
        )
    );
}