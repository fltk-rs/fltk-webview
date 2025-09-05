# fltk-webview

This provides webview functionality for embedded fltk windows.

## Usage
Add fltk-webview to your fltk application's Cargo.toml file:
```toml
[dependencies]
fltk = "1"
fltk-webview = "0.4"
```

Then you can embed a webview using fltk_webview::Webview::create:
```rust
use fltk::{app, prelude::*, window};
use fltk_webview::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.make_resizable(true);
    win.show();

    let mut wv = Webview::create(false, &mut wv_win);
    wv.navigate("https://google.com");
    
    app.run().unwrap();
}
```

## Dependencies
- fltk-rs's dependencies, which can be found [here](https://github.com/fltk-rs/fltk-rs#dependencies).
- On Windows: No other dependencies.
- On macOS: No other dependencies.
- On Linux (X11 only): WebKitGTK and X11 dev packages.
    - Debian-based distros: `sudo apt-get install libwebkit2gtk-4.1-dev libx11-dev`.
    - RHEL-based distros: `sudo dnf install webkit2gtk3-devel libX11-devel`.

## Linux Notes
- This crate supports X11 only on Linux. On GNOME or Wayland sessions, force X11: `GDK_BACKEND=x11`.
- GNOME/Mutter can interfere with embedded toplevels; the X11 path mitigates this by making the webview unmanaged.
- If you see blanking, try: `WEBKIT_DISABLE_COMPOSITING_MODE=1`.

![alt_test](screenshots/ex.jpg)

![alt_test](screenshots/markup.jpg)
