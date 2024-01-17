extern crate fltk;
extern crate tinyjson;

use fltk::{app, prelude::*, window};
use fltk_webview::*;
use tinyjson::JsonValue;

const HTML: &str = r#"data:text/html,
<html>
<body>hello</body>
<script>
    window.onload = function() {
        add(1, 2).then(function(res) {
            document.body.innerText = `added, ${res}`;
        });
        say_hello('Mo').then(function(res) {
            console.log(res);
        });
    };
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

    wv.bind("add", |seq, content| {
        println!("{}, {}", seq, content);
        let parsed: JsonValue = content.parse().unwrap();
        let val1: &f64 = parsed[0].get().unwrap();
        let val2: &f64 = parsed[1].get().unwrap();
        let ret = val1 + val2;
        // currenyly still not valid on MS Edge as well as the official for window.onload = function() {..},
        // binding (on first load window evaluation), but working fine on webkit.
        wv.return_(seq, 0, &ret.to_string());
    });

    wv.bind("say_hello", |seq, content| {
        println!("{}, {}", seq, content);
        let parsed: JsonValue = content.parse().unwrap();
        let val: &String = parsed[0].get().unwrap();
        wv.return_(seq, 0, &format!("Hello {}", val));
    });

    wv.navigate(HTML);
    app.run().unwrap();
}
