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
- On MacOS: No other dependencies.
- On X11/wayland platforms, webkit2gtk:
    - Debian-based distros: `sudo apt-get install libwebkit2gtk-4.1-dev`.
    - RHEL-based distros: `sudo dnf install webkit2gtk3-devel`.

## Known Issues
The situation on linux is quite bad. It depends on whether you're running X11 or wayland. On wayland, this will use xwayland. On X11, I can't get embedding to work on Gnome's mutter window manager, which keeps fighting for ownership of the webview window, causing flickering or a blank screen!D=x11 environment variable for webkit2gtk to work properly.

![alt_test](screenshots/ex.jpg)

![alt_test](screenshots/markup.jpg)
