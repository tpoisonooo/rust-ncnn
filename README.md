# rust-ncnn

ncnn Rust API.

# Prequisition

## Rust Env
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```


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

# Run Examples and UnitTest

```bash
$ cargo test
$ cargo run --example get_version
$ cargo run --example benchmark
```