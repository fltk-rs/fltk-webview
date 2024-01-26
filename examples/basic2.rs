use fltk::{app, prelude::*, group, input, window};
use fltk_webview::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut col = group::Flex::default_fill().column();
    col.set_margin(5);
    let inp = input::Input::default();
    col.fixed(&inp, 30);
    let mut wv_win = window::Window::default();
    col.end();
    win.end();
    win.make_resizable(true);
    win.show();

    let wv = Webview::create(false, &mut wv_win);
    wv.navigate("https://google.com");

    app.run().unwrap();
}
