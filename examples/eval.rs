extern crate fltk;
extern crate tinyjson;

use fltk::{app, prelude::*, window};
use fltk_webview::*;
use tinyjson::JsonValue;

const HTML: &str = r#"
<html>

<body>
    <div>
        <input id="inp" type="number" value=0>
    </div>
    <div>
        <button onclick="window.addTwo(parseFloat(document.getElementById('inp').value));">Add two!</button>
    </div>
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

    let wv = Webview::create(true, &mut wv_win);
    wv.bind("addTwo", |seq, content| {
        println!("{}, {}", seq, content);
        let parsed: JsonValue = content.parse().unwrap();
        let val1: &f64 = parsed[0].get().unwrap();
        let ret = val1 + 2.0;
        wv.eval(&format!(
            "document.getElementById('result').innerText = {}",
            ret
        )).unwrap();
    }).unwrap();

    wv.set_html(HTML).unwrap();

    app.run().unwrap();
}
