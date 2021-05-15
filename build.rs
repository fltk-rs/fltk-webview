#[cfg(target_os = "linux")]
fn main() {
    let cflags = std::process::Command::new("pkg-config")
        .args(&["--cflags", "gtk+-3.0"])
        .output()
        .expect("Needs pkg-config and gtk installed");
    let cflags = String::from_utf8_lossy(&cflags.stdout).to_string();
    let cflags: Vec<&str> = cflags.split_ascii_whitespace().collect();
    let mut build = cc::Build::new();
    build.file("src/gtkwid.c");
    for flag in cflags {
        build.flag(flag);
    }
    build
        .compile("gtkwid");
}

