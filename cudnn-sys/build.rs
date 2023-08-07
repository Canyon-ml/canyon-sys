
use std::env;
use std::path::PathBuf;

fn main() {

    let cudnn_root = env::var("CUDNN_ROOT").unwrap();
    let cuda_root = env::var("CUDA_ROOT").unwrap();

    // Tell Rustc where the cudnn library is
    println!("cargo:rustc-link-search={}/lib/", cudnn_root);
    // Tell Rustc the library to link (libcudnn.so)
    println!("cargo:rustc-link-lib=cudnn");
    // Tell Rustc where the cuda library is
    println!("cargo:rustc-link-search={}/lib64/", cuda_root);
    // Tell Rustc to link against cudart
    println!("cargo:rustc-link-lib=cudart");
    // Tell Cargo to rerun this build script if wrapper.h is changed
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include/", cudnn_root))
        .clang_arg(format!("-I{}/include/", cuda_root))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

