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

    let wv = Webview::create(false, &mut wv_win);
    wv.navigate("https://wikipedia.com").unwrap();

    app.run().unwrap();
}
