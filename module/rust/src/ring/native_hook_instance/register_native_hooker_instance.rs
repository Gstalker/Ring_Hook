use jni::JNIEnv;
use crate::ring::native_hook::NativeHookType::Inline;
use super::native_hook::{
    Manager,NativeHookConfig,InlineHookConfig,SymbolInfo,
};
use super::{
    android_log_print,
    register_dexfile,
};

pub fn register(env: &mut JNIEnv, path: String) -> bool {
    let mut result = true;
    info!("Register hooks");
    android_log_print::register(env,&path);
    register_dexfile::register(env,&path);
    info!("log hook finished!");
    result
}