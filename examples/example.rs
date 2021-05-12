use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.show();
    let mut wv = fltk_webview::from(false, &mut wv);
    wv.navigate("https://google.com");
    app.run().unwrap();
}
