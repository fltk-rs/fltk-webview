extern crate fltk;
extern crate tinyjson;

use fltk::{app, prelude::*, window};
use tinyjson::JsonValue;

const HTML: &str = r#"
data:text/html,
<!doctype html>
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

    let mut wv = fltk_webview::Webview::create(true, &mut wv_win);
    wv.navigate(HTML);

    let wvc = wv.clone();
    wv.bind("add", move |seq, content| {
        println!("{}, {}", seq, content);
        let parsed: JsonValue = content.parse().unwrap();
        let val1: &f64 = parsed[0].get().unwrap();
        let val2: &f64 = parsed[1].get().unwrap();
        let ret = val1 + val2;
        wvc.r#return(seq, 0, &ret.to_string());
    });

    let wvc = wv.clone();
    wv.bind("say_hello", move |seq, content| {
        println!("{}, {}", seq, content);
        let parsed: JsonValue = content.parse().unwrap();
        let val: &String = parsed[0].get().unwrap();
        wvc.r#return(seq, 0, &format!("Hello {}", val));
    });

    app.run().unwrap();
}
