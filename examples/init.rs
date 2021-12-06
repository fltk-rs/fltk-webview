extern crate fltk;
extern crate tinyjson;

use fltk::{app, button, prelude::*, window};

const HTML: &str = r#"<div id="result"></div>"#;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Webview");
    let mut wv_win = window::Window::new(5, 5, 390, 240, None);
    wv_win.end();
    let mut btn = button::Button::new(160, 255, 80, 30, "Click");
    win.end();
    win.make_resizable(true);
    win.show();

    let mut wv = fltk_webview::Webview::create(true, &mut wv_win);
    wv.init(r#"
    window.change = function() {
        let result = document.getElementById("result");
        result.innerText = "works";
    };
    "#);

    wv.set_html(HTML);

    btn.set_callback(move |_| wv.eval("window.change()"));

    app.run().unwrap();
}
