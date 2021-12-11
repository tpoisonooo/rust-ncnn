# rust-ncnn

ncnn Rust API.

# Prequisition

## CMake >= 3.12

Rust cmake needs `--parallel` option thus CMake3.12 is complusory

```bash
$ pip install cmake --upgrade --user
```

## Clang >= 3.9

Rust bindgen use `clang` to generate `bindings.rs` with `c_api.h`.

```bash
$ sudo apt install clang-3.9   libclang-3.9-dev
```

# Build

```bash
$ cd rust-ncnn/
$ cargo run --example get_version
```