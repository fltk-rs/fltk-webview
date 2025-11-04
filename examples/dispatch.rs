use fltk::{app, prelude::*, window};
use fltk_webview::*;

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

    let mut wv = Webview::create(true, &mut wv_win);
    wv.init(
        r#"
    var counter = (s) => {
        let result = document.getElementById("result");
        result.innerText = s;
    };
    "#,
    ).unwrap();
    wv.set_html(HTML).unwrap();

    let (s, r) = app::channel::<i32>();
    wv.dispatch(move |_wv| {
        std::thread::spawn(move || {
            let mut count = 0;
            loop {
                std::thread::sleep(std::time::Duration::from_millis(400));
                s.send(count);
                count += 1;
            }
        });
    }).unwrap();

    while app.wait() {
        if let Some(count) = r.recv() {
            wv.eval(&format!("counter({})", count)).unwrap();
        }
    }
}
