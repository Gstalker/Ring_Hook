use jni;
use std::ffi::CStr;

#[link(name = "classfinder")]
extern "C"{
    fn get_classfinder_methods() -> *mut jni::sys::JNINativeMethod;
}

pub fn get_classfinder_methods_vec() -> (String,Vec<jni::NativeMethod>) {
    let methods_ptr = unsafe{ get_classfinder_methods() };
    let methods_sys = unsafe{
        // length and capacity from module/src/main/cpp/yahfa/include/yahfa.h
        std::ptr::slice_from_raw_parts(methods_ptr,1).as_ref().unwrap()
    };
    let methods = methods_sys
        .iter()
        .map(|nm| jni::NativeMethod {
            name: jni::strings::JNIString::from(
                unsafe{ CStr::from_ptr(nm.name).to_str().unwrap().to_string() }
            ),
            sig: jni::strings::JNIString::from(
                unsafe{ CStr::from_ptr(nm.signature).to_str().unwrap().to_string() }
            ),
            fn_ptr: nm.fnPtr
        })
        .collect();
    ("bin.gstalker.ring.ClassFinder".to_string(),methods)
}
