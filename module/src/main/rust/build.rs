use std::{env,path::PathBuf};

fn get_cxx_static_lib_root_path() -> PathBuf {
    let cargo_root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let build_type = "release";
        // rust-android-gradle 暂不支持动态的cargo profile任务，见：
        // https://github.com/mozilla/rust-android-gradle
        // 由于module/build.gradle中设置cargo的build profile为debug
        // 故zipRelease任务编译的也是debug版本。
        // 目前解决的方案是，直接编译CPP的东西为Release版本，见module/build.gradle中设定cargo任务的部分
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

fn link_yahfa() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let arch_dir = match target_arch.as_str() {
        "arm" => "armeabi-v7a",
        "aarch64" => "arm64-v8a",
        "x86" => "x86",
        "x86_64" => "x86_64",
        _ => { panic!("do not support this arch") }
    };
    let cxx_static_lib_root = get_cxx_static_lib_root_path();
    let yahfa_static_lib = cxx_static_lib_root.join(arch_dir);
    // println!("cargo:warning=lib_path={}", yahfa_static_lib.display());
    println!("cargo:rustc-link-search={}",yahfa_static_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib=yahfa");
    println!("cargo:rustc-link-lib=dlfunc");
}

fn main() {
    link_yahfa()
}