use core::time;
use fltk::{app, prelude::*, window};
use fltk_webview::Webview;

// Work only on webkit, Egde doesn't.

const HTML: &str = r#"
<html>
<body>
    hello
    <div id="result">
        ok
    </div>
</body>
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
    wv.set_html(HTML);
    wv.init(
        r#"
    var counter = function(s) {
        let result = document.getElementById("result");
        result.innerText = s;
    };
    "#,
    );

    // wv.dispatch(|wv| {
    std::thread::spawn(move || {
        let mut count = 0;
        loop {
            std::thread::sleep(time::Duration::from_millis(400));
            wv.eval(&format!("counter({})", count));
            count += 1;
        }
    });
    // });
    app.run().unwrap();
}
