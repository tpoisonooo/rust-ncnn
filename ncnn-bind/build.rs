extern crate bindgen;
use cmake::Config;

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::str;

const DEFAULT_NCNN_TAG: &'static str = "20230517";

#[derive(PartialEq)]
enum VulkanMode {
    Static,
    System,
}

fn output_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn ncnn_src_dir() -> PathBuf {
    output_dir().join(format!("ncnn-src-{}", ncnn_tag()))
}

fn ncnn_tag() -> String {
    env::var("NCNN_TAG").unwrap_or(DEFAULT_NCNN_TAG.to_string())
}

fn fetch() -> io::Result<()> {
    let target_dir = ncnn_src_dir();

    let static_glslang = vulkan_mode().map_or_else(|| false, |mode| mode == VulkanMode::Static);

    if static_glslang {
        if target_dir.exists() {
            let stdout = Command::new("git")
                .arg("-C")
                .arg(&target_dir)
                .arg("submodule")
                .arg("foreach")
                .arg("true")
                .output()?
                .stdout;
            let out_text = String::from_utf8(stdout)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let submodule_rows = out_text.chars().filter(|c| *c == '\n').count();

            if submodule_rows > 0 {
                Ok(())
            } else {
                let status = Command::new("git")
                    .arg("-C")
                    .arg(&target_dir)
                    .arg("submodule")
                    .arg("update")
                    .arg("--init")
                    .arg("--recursive")
                    .status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        "submodule update failed",
                    ))
                }
            }
        } else {
            let status = Command::new("git")
                .arg("clone")
                .arg("--recursive")
                .arg("-b")
                .arg(ncnn_tag())
                .arg("https://github.com/Tencent/ncnn")
                .arg(&target_dir)
                .status()?;

            if status.success() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
            }
        }
    } else {
        if target_dir.exists() {
            Ok(())
        } else {
            let status = Command::new("git")
                .arg("clone")
                .arg("-b")
                .arg(ncnn_tag())
                .arg("https://github.com/Tencent/ncnn")
                .arg(&target_dir)
                .status()?;

            if status.success() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
            }
        }
    }
}

fn build() -> io::Result<()> {
    let mut config = Config::new(ncnn_src_dir());
    config.define("NCNN_BUILD_TOOLS", "OFF");
    config.define("NCNN_BUILD_EXAMPLES", "OFF");
    config.define("NCNN_BUILD_BENCHMARK", "OFF");
    config.define("NCNN_DISABLE_RTTI", "ON"); // Not used.
    config.define("NCNN_PIXEL_DRAWING", "OFF"); // Not exposed by ncnn-rs.
    config.define("NCNN_PIXEL_ROTATE", "OFF"); // Not exposed by ncnn-rs.
    config.define("NCNN_PIXEL_AFFINE", "OFF"); // Not exposed by ncnn-rs.
    config.define("CMAKE_BUILD_TYPE", "Release");

    if cfg!(feature = "openmp") {
        config.define("NCNN_OPENMP", "ON");
    } else {
        config.define("NCNN_OPENMP", "OFF");
    }

    if let Some(mode) = vulkan_mode() {
        config.define("NCNN_VULKAN", "ON");

        match mode {
            VulkanMode::Static => config.define("NCNN_SYSTEM_GLSLANG", "OFF"),
            VulkanMode::System => config.define("NCNN_SYSTEM_GLSLANG", "ON"),
        };
    }

    if use_dynamic_linking() {
        config.define("NCNN_SHARED_LIB", "ON");
    } else {
        config.define("NCNN_SHARED_LIB", "OFF");
    }

    let dst = config.build();

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

fn use_dynamic_linking() -> bool {
    if cfg!(feature = "static") && cfg!(feature = "dynamic") {
        panic!(
            "Both `static` and `dynamic` features are specified. Only one can be used at a time."
        );
    } else if cfg!(feature = "static") {
        false
    } else if cfg!(feature = "dynamic") {
        true
    } else {
        // By default use static linking for windows and dynamic for linux
        if cfg!(windows) {
            false
        } else {
            true
        }
    }
}

fn vulkan_mode() -> Option<VulkanMode> {
    match (
        cfg!(feature = "vulkan"),
        cfg!(feature = "vulkan-system-glslang"),
        cfg!(feature = "vulkan-static-glslang"),
    ) {
        (true, true, true) | (true, true, false) | (true, false, true) | (false, true, true) => {
            panic!("Cannot specify more than one `vulkan*` feature")
        }
        (true, false, false) => {
            if cfg!(windows) {
                Some(VulkanMode::Static)
            } else {
                Some(VulkanMode::System)
            }
        }
        (false, true, false) => Some(VulkanMode::System),
        (false, false, true) => Some(VulkanMode::Static),
        (false, false, false) => None,
    }
}

fn handle_openmp() {
    if cfg!(feature = "openmp") {
        // Maybe see https://gitlab.com/kornelski/openmp-rs/-/blob/master/build.rs ?
        //println!("cargo:rustc-link-lib=gomp");
        todo!()
    }
}

fn handle_vulkan() {
    if let Some(mode) = vulkan_mode() {
        let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
        let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();

        println!("cargo:rerun-if-env-changed=VULKAN_SDK");
        if let Ok(var) = env::var("VULKAN_SDK") {
            let suffix = match (&*target_family, &*target_pointer_width) {
                ("windows", "32") => "Lib32",
                ("windows", "64") => "Lib",
                _ => "lib",
            };
            println!("cargo:rustc-link-search={}/{}", var, suffix);
        }
        let lib = match &*target_family {
            "windows" => "vulkan-1",
            _ => "vulkan",
        };
        println!("cargo:rustc-link-lib={}", lib);

        match mode {
            VulkanMode::System => {
                println!("cargo:rustc-link-lib=dylib=SPIRV");
                println!("cargo:rustc-link-lib=dylib=glslang");

                if !use_dynamic_linking() {
                    // Glslang still depend on the C++ std.
                    println!("cargo:rustc-link-lib=dylib=stdc++");
                }
            }
            VulkanMode::Static => todo!(),
        }
    }
}

fn build_ncnn() -> Vec<PathBuf> {
    println!("cargo:rerun-if-env-changed=NCNN_DIR");
    println!("cargo:rerun-if-env-changed=NCNN_TAG");

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
            output_dir().join("lib").to_string_lossy()
        );

        vec![output_dir().join("include").join("ncnn")]
    };

    if use_dynamic_linking() {
        println!("cargo:rustc-link-lib=dylib=ncnn");
    } else {
        println!("cargo:rustc-link-lib=static=ncnn");
    }

    if !cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=pthread");
    }

    handle_openmp();

    handle_vulkan();

    return include_paths;
}

fn main() {
    let include_paths = if let Ok(vcpkg_lib) = vcpkg::find_package("ncnn") {
        vec![vcpkg_lib.include_paths[0].join("ncnn")]
    } else {
        build_ncnn()
    };

    let header = search_include(&include_paths, "c_api.h");

    let bindings = bindgen::Builder::default()
        .header(header)
        .allowlist_type("regex")
        .allowlist_function("ncnn.*")
        .allowlist_var("NCNN.*")
        .allowlist_type("ncnn.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(output_dir().join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
