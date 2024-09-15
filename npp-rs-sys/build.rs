use std::env;
use std::path::{Path, PathBuf};

fn find_dir(cuda_env_vars: &[&'static str], candidates: &[&'static str]) -> Option<PathBuf> {
    let mut set_variables = std::collections::HashMap::new();
    let mut set_paths = std::collections::HashSet::new();

    for env_var in cuda_env_vars {
        if let Some(cand_cuda_path) = env::var_os(env_var) {
            set_variables.insert(env_var, cand_cuda_path.clone());
            set_paths.insert(cand_cuda_path);
        }
    }

    match set_paths.len() {
        0 => {
            // No paths set, try defaults
            for candidate in candidates {
                let p = PathBuf::from(candidate);
                if p.is_dir() {
                    println!(
                        "cargo::warning=Using CUDA path from list of defaults: {}",
                        p.to_str().unwrap(),
                    );
                    return Some(p);
                }
            }
        }
        1 => {
            let (env_var, cuda_path) = set_variables.drain().next().unwrap();
            println!(
                "cargo::warning=Using CUDA path: {} from environment variable {}",
                cuda_path.to_str().unwrap(),
                env_var
            );
            return Some(PathBuf::from(cuda_path));
        }
        _ => {
            panic!(
                "ERROR: npp-rs-sys: Multiple CUDA paths set:

    {:?}

    npp-rs-sys does not know which cuda version should be linked to.",
                set_variables
            );
        }
    }

    None
}

fn validate_and_link_npp_install(cuda_home: &Path, npplibs: &[&str], static_link: bool) -> PathBuf {
    let cuda_include_dir = cuda_home.join("include");
    let npp_h = cuda_include_dir.join("npp.h");
    if !npp_h.is_file() {
        panic!(
            "ERROR: npp-rs-sys: Could not find npp.h include directory: {}",
            npp_h.to_string_lossy()
        );
    }

    let libdir = cuda_home.join("lib64");
    if libdir.is_dir() {
        println!("cargo:rustc-link-search={}", libdir.to_string_lossy());
    } else {
        panic!(
            "ERROR: npp-rs-sys: Could not find CUDA lib directory: {}",
            libdir.to_string_lossy()
        );
    }

    for npplib in npplibs {
        let libpath = if static_link {
            libdir.join(format!("lib{}_static.a", npplib))
        } else {
            libdir.join(format!("lib{}.so", npplib))
        };
        if !libpath.is_file() {
            panic!(
                "ERROR: npp-rs-sys: Could not find npp library library: {}",
                libpath.to_string_lossy()
            );
        }
        if static_link {
            println!("cargo:rustc-link-lib=static={}_static", npplib);
        } else {
            println!("cargo:rustc-link-lib=dylib={}", npplib);
        }
    }

    cuda_include_dir
}

fn main() {
    let cuda_path_vars = ["CUDA_PATH", "CUDA_HOME", "CUDA_ROOT", "CUDA_TOOLKIT_ROOT"];
    let cuda_default_paths = ["/opt/cuda", "/usr/local/cuda"];
    let npplibs = vec![
        "cudart", "nppc", "nppial", "nppicc", "nppidei", "nppif", "nppig", "nppim", "nppist",
        "nppisu", "nppitc", "npps",
    ];
    let static_link = cfg!(feature = "static-link");
    if static_link {
        println!("cargo::warning=npp-rs-sys: using static linking",);
    } else {
        println!("cargo::warning=npp-rs-sys: using dynamic linking",);
    }
    // change detection
    println!("cargo:rerun-if-changed=wrapper.h");
    for var in cuda_path_vars {
        println!("cargo:rerun-if-env-changed={}", var);
    }

    let cuda_home =
        find_dir(&cuda_path_vars, &cuda_default_paths).expect("Could not find CUDA path");

    let cuda_include = validate_and_link_npp_install(&cuda_home, &npplibs, static_link);

    // Setup bindgen to scan correct folders
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
