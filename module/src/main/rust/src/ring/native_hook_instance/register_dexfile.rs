use super::native_hook;
use super::native_hook::{
    NativeHookConfig,
    SymbolInfo
};
use std::mem::transmute;
use jni::JNIEnv;

#[inline(never)]
#[no_mangle]
pub extern "C" fn register_dex_file_hooker(thiz: *mut(),dexfile: *mut(), class_loader: *mut()) -> usize{
    let backup_result = native_hook::get_backup_trampoline_by_hook_function_addr(
            register_dex_file_hooker as usize
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
    let backup: extern "C" fn(*mut(),*mut(),*mut()) -> usize = unsafe{transmute(backup)};
    info!("hello_world from registerDex,0x{:p},0x{:p},0x{:p}",thiz,dexfile,class_loader);
    return backup(thiz,dexfile,class_loader);
}


#[allow(unused_variables)]
pub fn register(env: &mut JNIEnv,path :&String) {
    native_hook::register_native_hooker(
        NativeHookConfig::inline_config(
            register_dex_file_hooker as usize,
            None,
            Some(SymbolInfo::from_symbol_name_with_image_name(
                "_ZN3art11ClassLinker15RegisterDexFileERKNS_7DexFileENS_6ObjPtrINS_6mirror11ClassLoaderEEE".to_string(),
                "libart.so".to_string())),
            true
        )
    );
}