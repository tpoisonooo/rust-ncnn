# rust-ncnn
[![GitHub license](https://img.shields.io/badge/license-apache--2--Clause-brightgreen.svg)](./LICENSE) [![CI](https://img.shields.io/github/actions/workflow/status/tpoisonooo/rust-ncnn/ci.yaml?branch=master)](https://github.com/tpoisonooo/rust-ncnn/actions/workflows/ci.yaml?query=workflow%3A)

Rust bindings for [ncnn](https://github.com/tencent/ncnn).

## Docs

Open Github pages
* [ncnn_rs](https://rust-ncnn.github.io/ncnn_rs/) - low-level bindings
* [ncnn_bind](https://rust-ncnn.github.io/ncnn_bind/) - safe ncnn abstractions

Or `cargo doc` and open with browser byself

```bash
$ cd /path/to/rust-ncnn
$ cargo doc --open
```

## Prequisition

### Rust Env
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```


### CMake >= 3.12

Rust cmake needs `--parallel` option thus CMake>=3.12 is complusory

```bash
$ pip install cmake --upgrade --user
```

### Clang >= 3.9

Rust bindgen uses `clang` to generate `bindings.rs` from `c_api.h`

```bash
$ sudo apt install clang-3.9 libclang-3.9-dev
$ sudo apt install clang-10 libclang-10-dev # use clang-10 for ubuntu 20.04 
```

## Build

ncnn build from source:
```bash
$ cd rust-ncnn/
$ cargo run --example get_version
```

Use specific ncnn release:
```bash
$ export NCNN_TAG="20220420"
```

Use prebuilt ncnn:
```bash
$ export NCNN_DIR="/path/to/your/ncnn/lib"
```

Or use vcpkg
```bash
vcpkg install ncnn:x64-windows-static-md
cargo run --example get_version
```

## Linking

By default library uses dynamic **linking on linux** and **static linking on windows**.

To explicitly use static linking:
```bash
$ cargo build --example benchmark --features ncnn-bind/static
```

To explicitly use dynamic linking:
```bash
$ cargo build --example benchmark --features ncnn-bind/dynamic
```

## Vulkan

Build with Vulkan support (requires Vulkan SDK):
```bash
$ cargo build --example benchmark --features ncnn-bind/vulkan
```

## Run Examples and UnitTest

```bash
$ cargo test
$ cargo run --example get_version
$ cargo run --example benchmark --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/examples/benchmark`
squeezenet.param 		 2 ms
squeezenet_int8.param 		 5 ms
mobilenet.param 		 3 ms
mobilenet_int8.param 		 7 ms
mobilenet_v2.param 		 3 ms
mobilenet_v3.param 		 2 ms
shufflenet.param 		 2 ms
shufflenet_v2.param 		 2 ms
mnasnet.param 		 2 ms
proxylessnasnet.param 		 3 ms
efficientnet_b0.param 		 5 ms
regnety_400m.param 		 6 ms
blazeface.param 		 0 ms
googlenet.param 		 10 ms
googlenet_int8.param 		 19 ms
resnet18.param 		 9 ms
resnet18_int8.param 		 16 ms
alexnet.param 		 7 ms
vgg16.param 		 49 ms
vgg16_int8.param 		 71 ms
resnet50.param 		 18 ms
resnet50_int8.param 		 40 ms
squeezenet_ssd.param 		 17 ms
squeezenet_ssd_int8.param 		 13 ms
mobilenet_ssd.param 		 8 ms
mobilenet_ssd_int8.param 		 15 ms
mobilenet_yolo.param 		 30 ms
mobilenetv2_yolov3.param 		 13 ms
yolov4-tiny.param 		 20 ms
nanodet-plus-m_416.param 		 11 ms
nanodet-plus-m_416-int8.param 		 20 ms
```

## Acknowledgements

* [lit-robotics/rust-ncnn](https://github.com/lit-robotics/rust-ncnn)
* [ncnn](https://github.com/tencent/ncnn)
