use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=HIP_PATH");
    println!("cargo:rerun-if-env-changed=ROCM_PATH");

    let hip_path = root_candidates()
        .find(|path| path.join("include/hip/hip_runtime_api.h").is_file())
        .unwrap_or_else(|| {
            panic!(
                "Unable to find include path containing `hip/hip_runtime_api.h` under any of: {:?}.
                Set the `HIP_PATH` environment variable such that `$HIP_PATH/include/hip/hip_runtime_api.h` exists.",
                root_candidates().collect::<Vec<_>>()
            )
        });

    println!(
        "cargo:warning={}: Using '{}' as HIP_PATH",
        env!("CARGO_PKG_NAME"),
        hip_path.display()
    );

    let hip_lib = hip_path.join("lib");
    println!("cargo:rustc-link-search=native={}", hip_lib.display());
    println!("cargo:rustc-link-lib=dylib=amdhip64");

    #[cfg(feature = "bindgen")]
    {
        // The bindgen::Builder is the main entry point to bindgen, and lets you
        // build up options for the resulting bindings.
        println!("cargo:rerun-if-changed=wrapper.h");
        let bindings = bindgen::Builder::default()
            .raw_line("#![allow(non_camel_case_types)]")
            .raw_line("#![allow(non_upper_case_globals)]")
            .raw_line("#![allow(non_snake_case)]")
            // The input header we would like to generate bindings for.
            .header("wrapper.h")
            .clang_arg(format!("-I{}", hip_path.join("include").display()))
            .clang_arg(format!("-I{}", hip_path.join("include/hipify").display()))
            .clang_arg("-D__HIP_PLATFORM_AMD__") // needed for rocm>6)
            .rustified_non_exhaustive_enum("hip.*")
            .generate_block(false)
            .size_t_is_usize(true)
            .ctypes_prefix("::libc")
            .derive_default(true)
            .derive_eq(true)
            .derive_ord(true)
            .derive_hash(true)
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");
        bindings
            .write_to_file("src/bindings.rs")
            .expect("Couldn't write bindings!");
    }
}

/**
 * Possible locations for HIP path root
 */
#[allow(unused)]
fn root_candidates() -> impl Iterator<Item = PathBuf> {
    let env_vars = [
        "HIP_PATH",
        "ROCM_PATH", // on rocm>6, HIP_PATH is ROCM_PATH
    ].iter()
        .map(std::env::var)
        .filter_map(Result::ok)
        .map(Into::<PathBuf>::into);

    let roots = [
        "/opt/rocm",
        "/opt/rocm/hip",
    ].iter()
        .map(Into::<PathBuf>::into);
    env_vars.chain(roots)
}