use jni::JNIEnv;
use super::{
    register_dexfile,
};

pub fn register(env: &mut JNIEnv, path: String) -> bool {
    let result = true;
    info!("Register hooks");
    register_dexfile::register(env,&path);
    info!("log hook finished!");
    result
}