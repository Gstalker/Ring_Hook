use std::ffi::c_void;
use std::mem::transmute;
use jni::{
    objects::{
        JObject,
        JClass
    },
    JNIEnv,
    NativeMethod,
    strings::JNIString
};


use super::dex_loader::get_hooker_class_loader;

pub extern "system" fn JAVA_Ring_get_hooker_class_loader(env: JNIEnv, clz: JClass) -> JObject<'static> {
    unsafe{ transmute(get_hooker_class_loader().clone().unwrap().as_obj()) }
}

pub fn get_methods() -> (String,Vec<NativeMethod>) {
    let mut methods = Vec::new();
    methods.push( NativeMethod{
        name: JNIString::from("getHookerClassLoader"),
        sig: JNIString::from("()Ljava/lang/ClassLoader;"),
        fn_ptr: JAVA_Ring_get_hooker_class_loader as *mut c_void
    });
    ("bin.gstalker.ring.HookManager".to_string(),methods)
}