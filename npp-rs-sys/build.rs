use std::env;
use std::path::PathBuf;

fn find_dir(env_key: &'static str, candidates: Vec<&'static str>) -> Option<PathBuf> {
    match env::var_os(env_key) {
        Some(val) => Some(PathBuf::from(&val)),
        _ => {
            None
        }
    }
}

fn main() {
    let cuda_home = find_dir(
        "CUDA_HOME",
        vec!["/opt/cuda", "/usr/local/cuda"],
    ).expect("Could not find CUDA path");
    let cuda_include = cuda_home.join("include");

    println!("cargo:rustc-link-search={}/lib64", cuda_home.to_string_lossy());

    let libraries = vec![
        "cudart",
        "nppc",
        "nppial",
        "nppicc",
        "nppidei",
        "nppif",
        "nppig",
        "nppim",
        "nppist",
        "nppisu",
        "nppitc",
        "npps",
    ];
    for library in libraries {
        println!("cargo:rustc-link-lib=dylib={}", library);
    }

    //println!("cargo:rustc-link-lib=culibos");
    //println!("cargo:rustc-link-lib=dylib=stdc++");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", cuda_include.to_string_lossy()))
        .header("wrapper.h")
        .blocklist_function("strtold")
        .blocklist_function("qecvt")
        .blocklist_function("qfcvt")
        .blocklist_function("qgcvt")
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt_r")
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
