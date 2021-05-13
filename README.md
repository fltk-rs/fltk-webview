# fltk-webview

This provides webview functionality for embedded fltk windows. This currently works on Windows:

## Usage

```rust
extern crate fltk;

use fltk::{app, enums::Event, prelude::*, window};

fn main() {
    let _app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.show();

    // close the app when the main window is closed
    win.set_callback(|_| {
        if app::event() == Event::Close {
            std::process::exit(0);
        }
    });

    let mut wv = fltk_webview::Webview::create(false, &mut wv_win);
    wv.navigate("http://wikipedia.com");
    
    // the webview handles the main loop
    wv.run();
}
```

## Limitations
- On windows, webview requires winrt headers, that means it's basically buildable with the MSVC toolchain. For Msys2/mingw, there are efforts to provide such headers, but nothing yet upstream.
- On macos, unhandled objective-c exceptions can lead to faulty behavior.
- On linux, I can't construct a GtkWindow from an FLTK window nor from an FLTK raw handle (xid). If you're able to do so, your help is needed!


![alt_test](screenshots/ex.jpg)
