use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    compile_webview();

    #[cfg(target_os = "macos")]
    compile_cocoa_helper();
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
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

fn compile_webview() {
    let target = env::var("TARGET").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let exe_pth = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .current_dir(&manifest_dir)
        .status()
        .expect("Git is needed to retrieve the fltk & webview source files!");

    println!("cargo:rerun-if-changed=webview/webview.h");
    println!("cargo:rerun-if-changed=webview/webview.cc");
    println!("cargo:rerun-if-changed=src/gtk_helper.c");
    println!("cargo:rerun-if-changed=src/cocoa_helper.m");

    let mut build = cc::Build::new();
    if !target.contains("windows-gnu") {
        build
            .cpp(true)
            .file("webview/webview.cc")
            .flag_if_supported("-w");
    }

    if target.contains("windows") {
        let edge_weview_native =
            "webview/script/Microsoft.Web.WebView2.1.0.1343.22/build/native".to_string();
        if target.contains("msvc") {
            let mut include = edge_weview_native.clone();
            include.push_str("/include");
            build.flag("/DWEBVIEW_EDGE");
            build.flag("/std:c++17");
            build.include(include);
        }

        for &lib in &[
            "user32",
            "oleaut32",
            "ole32",
            "version",
            "shell32",
			"advapi32",
			"shlwapi"
        ] {
            println!("cargo:rustc-link-lib={}", lib);
        }

        let wv_arch = if target.contains("x86_64") {
            "x64"
        } else if target.contains("i686") {
            "x86"
        } else {
            "arm64"
        };

        let mut wv_path = manifest_dir;
        if target.contains("msvc") {
            wv_path.push(edge_weview_native);
        } else {
            wv_path.push("webview");
            wv_path.push("dll");
        }
        wv_path.push(wv_arch);
        let webview2_dir = wv_path.as_path().to_str().unwrap();
        println!("cargo:rustc-link-search={}", webview2_dir);
        if target.contains("msvc") {
            println!("cargo:rustc-link-lib=WebView2LoaderStatic");
        } else {
            if !target.contains("aarch64") {
                println!("cargo:rustc-link-lib=WebView2Loader");
                println!("cargo:rustc-link-lib=webview");
                for entry in std::fs::read_dir(wv_path).expect("Can't read DLL dir") {
                    let entry_path = entry.expect("Invalid fs entry").path();
                    let file_name_result = entry_path.file_name();
                    let mut exe_pth = exe_pth.clone();
                    if let Some(file_name) = file_name_result {
                        let file_name = file_name.to_str().unwrap();
                        if file_name.ends_with(".dll") {
                            exe_pth.push("../../..");
                            let mut for_examples_exe_pth = exe_pth.clone();
                            for_examples_exe_pth.push("examples");
                            exe_pth.push(file_name);
                            std::fs::copy(&entry_path, exe_pth.as_path())
                                .expect("Can't copy from DLL dir /target/..");

                            // Copy .dll to examples folder too, in order to run examples when cross compiling from linux.
                            for_examples_exe_pth.push(file_name);
                            std::fs::copy(&entry_path, for_examples_exe_pth.as_path())
                                .expect("Can't copy from DLL dir to /target/../examples");
                        }
                    }
                }
            } else {
                panic!("{:?} not supported yet", target)
            }
        }
    } else if target.contains("apple") {
        build.flag("-DWEBVIEW_COCOA");
        build.flag("-std=c++11");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=WebKit");
    } else if target.contains("linux") || target.contains("bsd") {
        build.flag("-DWEBVIEW_GTK");
        build.flag("-std=c++11");
        let lib = pkg_config::Config::new()
            .atleast_version("2.8")
            .probe("webkit2gtk-4.0")
            .unwrap();
        for path in lib.include_paths {
            build.include(path);
        }
	#[cfg(not(any(target_os = "macos", target_os = "windows")))]
        compile_gtk_helper();
    } else {
        panic!("Unsupported platform");
    }

    if !target.contains("windows-gnu") {
        build.compile("webview");
    }
}
