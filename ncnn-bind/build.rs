extern crate bindgen;
use cmake::Config;

use std::env;
use std::fs::{self};
use std::io::{self};
use std::path::PathBuf;
use std::process::Command;
use std::str;

fn fetch() -> io::Result<()> {
    let output_base_path = output();
    let clone_dest_dir = "ncnn-master";

    let target_dir = output_base_path.join(&clone_dest_dir);
    if target_dir.exists() {
        return Ok(());
    }
    let _ = std::fs::remove_dir_all(output_base_path.join(&clone_dest_dir));
    let status = Command::new("git")
        .current_dir(&output_base_path)
        .arg("clone")
        .arg("--depth=1")
        .arg("-b")
        .arg("rust-ncnn")
        .arg("https://github.com/tpoisonooo/ncnn")
        .arg(&clone_dest_dir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
    }
}

fn build() -> io::Result<()> {
    let dst;
    if env::var("CARGO_FEATURE_STATIC").is_ok() {
        dst = Config::new(ncnndir())
            .define("NCNN_BUILD_TOOLS", "OFF")
            .define("NCNN_BUILD_EXAMPLES", "OFF")
            .define("CMAKE_BUILD_TYPE", "Release")
            .define("NCNN_VULKAN", "OFF")
            // .define(
            //     "CMAKE_TOOLCHAIN_FILE",
            //     ncnndir()
            //         .join("toolchains/host.gcc.toolchain.cmake")
            //         .to_str()
            //         .unwrap(),
            // )
            .cflag("-std=c++14")
            .build();
    } else {
        dst = Config::new(ncnndir())
            .define("NCNN_BUILD_TOOLS", "OFF")
            .define("NCNN_BUILD_EXAMPLES", "OFF")
            .define("NCNN_SHARED_LIB", "ON")
            .define("NCNN_VULKAN", "OFF")
            .define("CMAKE_BUILD_TYPE", "Release")
            // .define(
            //     "CMAKE_TOOLCHAIN_FILE",
            //     ncnndir()
            //         .join("toolchains/host.gcc.toolchain.cmake")
            //         .to_str()
            //         .unwrap(),
            // )
            .cflag("-std=c++14")
            .build();
    }
    println!("cargo:rustc-link-search=native={}", dst.display());
    Ok(())
}

fn search_include(include_paths: &[PathBuf], header: &str) -> String {
    for dir in include_paths {
        let include = dir.join(header);
        if fs::metadata(&include).is_ok() {
            return include.as_path().to_str().unwrap().to_string();
        }
    }
    format!("/usr/include/{}", header)
}

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn ncnndir() -> PathBuf {
    output().join("ncnn-master")
}

fn link_to_libraries() {
    if env::var("CARGO_FEATURE_STATIC").is_ok() {
        println!("cargo:rustc-link-lib=static=ncnn");
    } else {
        println!("cargo:rustc-link-lib=dylib=ncnn");
    }
    println!("cargo:rustc-link-lib=dylib=pthread");
}

fn main() {
    let include_paths: Vec<PathBuf> = if let Ok(ncnn_dir) = env::var("NCNN_DIR") {
        // use prebuild ncnn dir
        let dir = PathBuf::from(ncnn_dir);
        println!(
            "cargo:rustc-link-search=native={}",
            dir.join("lib").to_string_lossy()
        );

        vec![dir.join("include").join("ncnn")]
    } else {
        // fetch from github and build
        fetch().unwrap();
        build().unwrap();

        println!(
            "cargo:rustc-link-search=native={}",
            output().join("lib").to_string_lossy()
        );

        vec![output().join("include").join("ncnn")]
    };

    link_to_libraries();
    let mut builder = bindgen::Builder::default();

    let files = vec!["c_api.h"];
    for file in files {
        builder = builder.header(search_include(&include_paths, file));
    }

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = output();
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
