# hip-sys
Rust bindings for HIP

# Dependencies
  - A ROCm platform ie a compatible AMD GPU
    * `hipblas` can be accessed with the `blas` feature
    * Specify the path to `hip` with `HIP_PATH`. If not provided,
      `hipconfig -p` is tried or `/opt/rocm/hip` is assumed
    * Specify the path to `hipblas` with `HIP_BLAS_PATH`. If not provided,
      `/opt/rocm/hipblas` is assumed
    * Native bindings can be generated with the `bindgen` feature
    * Will compile without an AMD GPU, but device functions will fail
    * Currently CUDA platform support is not available (see cuda-sys https://github.com/rust-cuda/cuda-sys)

## Install ROCm
  - Ubuntu: See https://rocmdocs.amd.com/en/latest/deploy/linux/quick_start.html
    * Works with the hcc backend `sudo apt install hip-hcc` (from ROCm package registry)
    - You may need to install rocsolver as well `sudo apt install rocsolver`
  - Arch: See https://github.com/arch4edu/arch4edu
    - `sudo pacman -S rocm-hip-runtime`

If you have any problems please post an issue.

# Tests
Run the tests with:
```
cargo test
```

# Docs
Open the documentation with:
```
cargo doc --open
```

# Acknowledgement
This repo is derived from one created
[here](https://github.com/charles-r-earp/hip-sys); permission has been gained to
publish this work with local modifications (see issue #1).
