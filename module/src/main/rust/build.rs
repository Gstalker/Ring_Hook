use std::{env,path::PathBuf};

fn get_cxx_static_lib_root_path() -> PathBuf {
    let cargo_root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let build_type = env::var_os("PROFILE").unwrap().into_string().unwrap();
    let lib_root = cargo_root
        .join("..")
        .join("..")
        .join("..")
        .join("build")
        .join("intermediates")
        .join("cmake")
        .join(build_type)
        .join("obj");
    lib_root
}

#[cfg(target_arch = "x86_64")]
fn link_yahfa() {
    let cxx_static_lib_root = get_cxx_static_lib_root_path();
    let yahfa_static_lib = cxx_static_lib_root
        .join("x86_64")
        .join(".");
    println!("cargo:rustc-link-search={}",yahfa_static_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib={}","yahfa");
}

#[cfg(target_arch = "x86")]
fn link_yahfa() {
    let cxx_static_lib_root = get_cxx_static_lib_root_path();
    let yahfa_static_lib = cxx_static_lib_root
        .join("x86")
        .join(".");
    println!("cargo:rustc-link-search={}",yahfa_static_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib={}","yahfa");
}

#[cfg(target_arch = "armv7")]
fn link_yahfa() {
    let cxx_static_lib_root = get_cxx_static_lib_root_path();
    let yahfa_static_lib = cxx_static_lib_root
        .join("armeabi-v7a")
        .join(".");
    println!("cargo:rustc-link-search={}",yahfa_static_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib={}","yahfa");
}

#[cfg(target_arch = "aarch64")]
fn link_yahfa() {
    let cxx_static_lib_root = get_cxx_static_lib_root_path();
    let yahfa_static_lib = cxx_static_lib_root
        .join("arm64-v8a")
        .join(".");
    println!("cargo:rustc-link-search={}",yahfa_static_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib={}","yahfa");
}

fn main() {
    link_yahfa()
}