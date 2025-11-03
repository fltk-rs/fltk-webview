use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/gtk_helper.c");
    println!("cargo:rerun-if-changed=src/cocoa_helper.m");
    println!("cargo:rerun-if-changed=src/win32_helper.c");

    #[cfg(target_os = "macos")]
    compile_cocoa_helper();

    #[cfg(target_os = "linux")]
    compile_gtk_helper();

    #[cfg(target_os = "windows")]
    compile_win32_helper();
}

#[cfg(target_os = "linux")]
fn compile_gtk_helper() {
    let cflags = std::process::Command::new("pkg-config")
        .args(&["--cflags", "gtk+-3.0"])
        .output()
        .expect("Needs pkg-config and gtk installed");
    let cflags = String::from_utf8_lossy(&cflags.stdout).to_string();
    let cflags: Vec<&str> = cflags.split_ascii_whitespace().collect();
    let mut build = cc::Build::new();
    build.file("src/gtk_helper.c");
    for flag in cflags {
        build.flag(flag);
    }
    build.compile("gtkwid");
}

#[cfg(target_os = "macos")]
fn compile_cocoa_helper() {
    let mut build = cc::Build::new();
    build.file("src/cocoa_helper.m");
    build.compile("cocoa");
}

#[cfg(target_os = "windows")]
fn compile_win32_helper() {
    let wv_sys_path = find_wv_sys_path();
    let mut build = cc::Build::new();
    build.file("src/win32_helper.c");
    build.include(&wv_sys_path.join("libs\\include"));
    build.include(&wv_sys_path.join("webview\\core\\include"));
    build.compile("windows");
}

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}
#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    manifest_path: PathBuf,
}

fn find_wv_sys_path() -> PathBuf {
    let output = Command::new("cargo")
        .arg("metadata")
        .output()
        .expect("Failed to run cargo metadata");

    let metadata: Metadata =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata");

    let wv_sys_pkg = metadata
        .packages
        .into_iter()
        .find(|pkg| pkg.name == "wv-sys")
        .expect("Could not find 'wv-sys' package in metadata");

    let path = wv_sys_pkg
        .manifest_path
        .parent()
        .expect("manifest_path has no parent");

    if !path.exists() {
        panic!(
            "Failed to find headers at: {}. wv-sys internal layout may have changed.",
            path.display()
        );
    }

    path.to_path_buf()
}
