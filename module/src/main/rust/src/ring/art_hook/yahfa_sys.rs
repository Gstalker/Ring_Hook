use jni;
use std::sync::Mutex;
use std::ffi::CString;

#[link(name = "yahfa")]
extern "C"{
    fn get_yahfa_hook_main_methods() -> *mut jni::sys::JNINativeMethod;
    fn get_yahfa_utils_methods() -> *mut jni::sys::JNINativeMethod;
}

pub fn get_yahfa_hook_main_methods_vec() -> (String,Vec<jni::NativeMethod>) {
    let yahfa_hook_main_methods_ptr = unsafe{ get_yahfa_hook_main_methods() };
    let yahfa_hook_main_methods_sys :Vec<jni::sys::JNINativeMethod> = unsafe{
        // length and capacity from module/src/main/cpp/yahfa/include/yahfa.h
        Vec::from_raw_parts(yahfa_hook_main_methods_ptr,3 ,3)
    };
    let yahfa_hook_main_methods = yahfa_hook_main_methods_sys
        .iter()
        .map(|nm| jni::NativeMethod {
            name: jni::strings::JNIString::from(
                unsafe{ CString::from_raw(nm.name).into_string().unwrap() }
            ),
            sig: jni::strings::JNIString::from(
                unsafe{ CString::from_raw(nm.signature).into_string().unwrap() }
            ),
            fn_ptr: nm.fnPtr
        })
        .collect();
    ("lab.galaxy.yahfa.HookMain".to_string(),yahfa_hook_main_methods)
}

pub fn get_yahfa_utils_native_methods_vec() -> (String, Vec<jni::NativeMethod>) {
    let yahfa_utils_methods_ptr = unsafe{ get_yahfa_utils_methods() };
    let yahfa_utils_methods_sys = unsafe{
        // length and capacity from module/src/main/cpp/yahfa/include/yahfa.h
        Vec::from_raw_parts(yahfa_utils_methods_ptr, 3, 3)
    };
    let yahfa_utils_native_methods = yahfa_utils_methods_sys
        .iter()
        .map(|nm| jni::NativeMethod {
            name: jni::strings::JNIString::from(
                unsafe{ CString::from_raw(nm.name).into_string().unwrap() }
            ),
            sig: jni::strings::JNIString::from(
                unsafe{ CString::from_raw(nm.signature).into_string().unwrap() }
            ),
            fn_ptr: nm.fnPtr,
        })
        .collect();
    ("lab.galaxy.yahfa.HookMain$Utils".to_string(), yahfa_utils_native_methods)
}