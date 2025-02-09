# GPU COMPUTE DRAFT

(Important) All these examples of programs with GPU have only been tested in:

OS
* Ubuntu Ubuntu 22.04.5 LTS 64-bit

GPU
* Radeon rx 6500xt
* Radeon rx 6600

CPU
* ryzen 5700G

ROCM
* 6.2.2.60202-116~22.04

Other
* At the moment it mostly requires the explanation of the code and its final objectives (I seek to somehow justify a certain madness of the present code)

---

# Crates

## hsakmt-rs

ROCt Thunk Library (`libhsakmt`) rewrite from C to Rust

## hsa-rs (TODO)

The HSA Runtime (`hsa-runtime`) rewrite from C++ to Rust

## amdgpu-drm-sys

Rust bindings of AMD (amdgpu, amdgpu_drm)

---

# Testing

test
```bash
cargo test -- --test-threads=1
```



https://internals.rust-lang.org/t/get-the-offset-of-a-field-from-the-base-of-a-struct/14163