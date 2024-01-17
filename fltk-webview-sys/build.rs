fn main() {
    println!("cargo:rerun-if-changed=src/gtk_helper.c");
    println!("cargo:rerun-if-changed=src/cocoa_helper.m");

    #[cfg(target_os = "macos")]
    compile_cocoa_helper();

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    compile_gtk_helper();
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
