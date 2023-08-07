
use std::env;
use std::path::PathBuf;

fn main() {

    let cuda_root = env::var("CUDA_ROOT").unwrap();
    let cudnn_root = env::var("CUDNN_ROOT").unwrap();

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
        .header("cudnn_wrapper.h")
        .clang_arg(format!("-I{}/include/", cuda_root))
        .clang_arg(format!("-I{}/include/", cudnn_root))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let out_path = PathBuf::from("src/cudnn_bindings.rs");
    bindings
        .write_to_file(out_path.join(format!("{}/bindings.rs", out_dir.display())))
        .expect("Couldn't write cudnn bindings!");

    let bindings = bindgen::Builder::default()
        .header("cuda_wrapper.h")
        .clang_arg(format!("-I{}/include/", cuda_root))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate cuda bindings");

    let out_path = PathBuf::from("src/cuda_bindings.rs");
    bindings
        .write_to_file(out_path.join(format!("{}/bindings.rs", out_dir.display())))
        .expect("Couldn't write cuda bindings!");
        
}

