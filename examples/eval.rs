extern crate fltk;
extern crate tinyjson;

use fltk::{app, prelude::*, window};
use tinyjson::JsonValue;

const HTML: &str = r#"data:text/html,
<!doctype html>
<html>
<input id="inp"><br>
<button onclick="window.addTwo(parseInt(document.getElementById('inp').value));">Add two!</button>
<div id="result"></div>
</body>
<script>
</script>
</html>"#;

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

    let mut wv = fltk_webview::Webview::create(true, &mut wv_win);

    let mut wvc = wv.clone();
    wv.bind("addTwo", move |seq, content| {
        println!("{}, {}", seq, content);
        let parsed: JsonValue = content.parse().unwrap();
        let val1: &f64 = parsed[0].get().unwrap();
        let ret = val1 + 2.0;
        wvc.eval(&format!(
            "document.getElementById('result').innerText = {}",
            ret
        ));
    });

    wv.navigate(HTML);

    app.run().unwrap();
}
