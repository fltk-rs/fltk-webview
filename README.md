# fltk-webview

This provides webview functionality for embedded fltk windows.
The webview bindings are based on the [webview-official-sys crate](https://crates.io/crates/webview-official-sys), which was modified for use with FLTK and to use the static WebView2Loader library on Windows along with a newer version of webview.

## Usage
Add fltk-webview to your fltk application's Cargo.toml file:
```toml
[dependencies]
fltk = "1"
fltk-webview = "0.2"
```

Then you can embed a webview using fltk_webview::Webview::create:
```rust
use fltk::{app, prelude::*, window};

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

    let mut wv = fltk_webview::Webview::create(false, &mut wv_win);
    wv.navigate("https://google.com");
    
    app.run().unwrap();
}
```

## Dependencies
- fltk-rs's dependencies, which can be found [here](https://github.com/fltk-rs/fltk-rs#dependencies).
- On Windows: No other dependencies.
- On MacOS: No other dependencies.
- On X11/wayland platforms, webkit2gtk:
    - Debian-based distros: `sudo apt-get install libwebkit2gtk-4.0-dev`.
    - RHEL-based distros: `sudo dnf install webkit2gtk3-devel`.

## Known Issues
- On windows, building using the gnu toolchain will still require deploying the dlls (webview.dll and WebView2Loader.dlls, which can be found in the `target/<profile>` directory.
- On X11/wayland platforms:
    - Need help with Gnome's mutter window manager fighting for ownership of the webview window, causing flickering in text fields!
    - If running on Wayland, you need to pass the GDK_BACKEND=x11 environment variable for webkit2gtk to work properly.


![alt_test](screenshots/ex.jpg)

![alt_test](screenshots/markup.jpg)
