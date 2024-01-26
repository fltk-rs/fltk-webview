#![doc = include_str!("../README.md")]

use fltk::{
    app, enums,
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window,
};
use fltk_webview_sys as wv;
pub use wv::Webview;
use std::{
    os::raw,
    sync::Arc,
};

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn win_manager(prog: &str) -> bool {
    let sm = std::env::var("SESSION_MANAGER");
    if let Ok(sm) = sm {
        let pid = sm.split("/").last();
        if let Some(pid) = pid {
            match std::process::Command::new("ps")
                .args(&["-p", pid, "-o", "comm="])
                .output()
            {
                Ok(out) => {
                    if String::from_utf8_lossy(&out.stdout).contains(prog) {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub trait FromFltkWindow {
    fn create(debug: bool, win: &mut window::Window) -> Webview;
}

impl FromFltkWindow for Webview {
    /// Create a Webview from an embedded fltk window. Requires that the window is already shown
    fn create(debug: bool, win: &mut window::Window) -> Webview {
        assert!(win.shown());
        win.end();
        win.set_color(enums::Color::White);
        let inner;
        unsafe {
            #[cfg(target_os = "windows")]
            {
                extern "system" {
                    pub fn SetFocus(child: *mut ()) -> *mut ();
                }
                inner = wv::webview_create(
                    debug as i32,
                    &mut win.raw_handle() as *mut *mut raw::c_void as *mut raw::c_void,
                );
                win.draw(move |w| wv::webview_set_size(inner, w.w(), w.h(), 0));
                let mut topwin =
                    window::Window::from_widget_ptr(win.top_window().unwrap().as_widget_ptr());
                SetFocus(topwin.raw_handle() as _);
                topwin.set_callback(|t| {
                    if app::event() == enums::Event::Close {
                        t.hide();
                    }
                });
                topwin.assume_derived();
                topwin.handle(|w, ev| match ev {
                    fltk::enums::Event::Push => {
                        SetFocus(w.raw_handle() as _);
                        true
                    },
                    _ => false
                });
            }
            #[cfg(target_os = "macos")]
            {
                pub enum NSWindow {}
                extern "C" {
                    pub fn make_delegate(child: *mut NSWindow, parent: *mut NSWindow, add_menu: i32);
                    pub fn my_close_win(win: *mut NSWindow);
                }
                let handle = win.raw_handle();
                inner = wv::webview_create(debug as i32, handle as _);
                make_delegate(wv::webview_get_window(inner) as _, handle as _, 1);
                win.draw(move |w| wv::webview_set_size(inner, w.w(), w.h(), 0));
                let mut topwin =
                    window::Window::from_widget_ptr(win.top_window().unwrap().as_widget_ptr());
                let inner = inner.clone();
                topwin.set_callback(move |t| {
                    if app::event() == enums::Event::Close {
                        my_close_win(wv::webview_get_window(inner) as _);
                        t.hide();
                    }
                });
            }
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            {
                pub enum GdkWindow {}
                pub enum GtkWindow {}
                pub enum Display {}
                extern "C" {
                    pub fn gtk_init(argc: *mut i32, argv: *mut *mut raw::c_char);
                    pub fn my_get_win(wid: *mut GtkWindow) -> *mut GdkWindow;
                    pub fn my_get_xid(w: *mut GdkWindow) -> u64;
                    pub fn x_init(disp: *mut Display, child: u64, parent: u64);
                    pub fn gtk_main_iteration_do(blocking: bool);
                }
                gtk_init(&mut 0, std::ptr::null_mut());
                inner = wv::webview_create(debug as i32, std::ptr::null_mut() as _);
                assert!(!inner.is_null());
                let temp_win = wv::webview_get_window(inner);
                assert!(!temp_win.is_null());
                let temp = my_get_win(temp_win as _);
                assert!(!temp.is_null());
                let xid = my_get_xid(temp as _);
                let flxid = win.raw_handle();
                if win_manager("gnome-session") {
                    win.draw(move |w| {
                        x_init(app::display() as _, xid, flxid);
                        app::sleep(0.03);
                        wv::webview_set_size(inner, w.w(), w.h(), 0);
                    });
                    win.flush();
                } else {
                    x_init(app::display() as _, xid, flxid);
                    win.draw(move |w| wv::webview_set_size(inner, w.w(), w.h(), 0));
                }

                app::add_timeout3(0.001, |handle| {
                    gtk_main_iteration_do(false);
                    app::repeat_timeout3(0.001, handle);
                });
            }
        }
        assert!(!inner.is_null());
        let inner = Arc::new(inner);
        Webview::from_raw(inner)
    }
}
