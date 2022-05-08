use std::ffi::{c_void, CStr};
use jni::{
    NativeMethod,
    sys,
    JNIEnv,
    strings::JNIString,
    objects::JString,
};

use super::native_hook;
use super::native_hook::{
    NativeHookConfig,
    SymbolInfo,
};
use super::art_hook::{
    register_hooker_native_functions,
    find_class,
};
use std::sync::Mutex;
use std::mem::transmute;


fn process_java_hookers(env: &JNIEnv) {
    let target_class = find_class(env,"bin.gstalker.ring.HookManager".to_string()).unwrap();
    env.call_static_method(
        target_class,
        "processHookers",
        "()V",
        &[]
    ).unwrap();
}

pub extern "C" fn find_loaded_class_hooker(
    env: JNIEnv, clz: sys::jclass, loader: sys::jobject, name: JString
) -> usize {
    let backup_result = native_hook::get_backup_trampoline_by_hook_function_addr(
        find_loaded_class_hooker as usize
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

    let backup: extern "C" fn(*mut(),*mut(),*mut(),*mut()) -> usize = unsafe{ transmute(backup) };
    let mut enable = false;
    let mut c_name = env.get_string_utf_chars(name).unwrap();
    let class_name = unsafe{ CStr::from_ptr(c_name).to_str().unwrap().to_string() };
    env.release_string_utf_chars(name,c_name);

    warn!("find_loaded_class hit! target class name: {}", class_name);{
        let mut status_guard = ACTIVATE_MARK.lock().unwrap();
        if *status_guard == true {
            enable = true;
            *status_guard = false;
            warn!("Process remaining java hookers!");
        }
    }
    if enable {
        process_java_hookers(&env);
    }

    return backup(
        env.get_native_interface() as *mut(),
        clz as *mut(),
        loader as *mut(),
        name.into_inner() as *mut())
}


pub extern "C" fn ClassLoaderHooker_setDefineClassNativeHookerReady(env: sys::JNIEnv, clz: sys::jclass) {
    *(ACTIVATE_MARK.lock().unwrap()) = true;
}

#[allow(unused_variables)]
pub fn register(env: &mut JNIEnv,path :&String) {
    native_hook::register_native_hooker(
        NativeHookConfig::inline_config(
            find_loaded_class_hooker as usize,
            None,
            Some(SymbolInfo::from_symbol_name_with_image_name(
                "_ZN3artL29VMClassLoader_findLoadedClassEP7_JNIEnvP7_jclassP8_jobjectP8_jstring".to_string(),
                "libart.so".to_string())),
            true
        )
    );
    register_hooker_native_functions(
        env,
        vec![NativeMethod{
            name: JNIString::from("setDefineClassNativeHookerReady"),
            sig: JNIString::from("()V"),
            fn_ptr: ClassLoaderHooker_setDefineClassNativeHookerReady as *mut c_void
        }],
        "bin.gstalker.ring.hooker.ClassLoaderHooker".to_string()
    );
}

lazy_static!{
    static ref ACTIVATE_MARK: Mutex<bool> = Mutex::new(false);
}