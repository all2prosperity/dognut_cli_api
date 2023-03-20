use std::env;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    let target = env::var("TARGET").unwrap();
    if target.starts_with("aarch64-linux-android") {
        println!("cargo:rustc-link-search=native=dep/android/arm64-v8a");
    }

    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avdevice");
    println!("cargo:rustc-link-lib=avfilter");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avresample");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=swresample");
    println!("cargo:rustc-link-lib=swscale");
    println!("cargo:rustc-link-lib=postproc");
    // Use the `cc` crate to build a C file and statically link it.
}
