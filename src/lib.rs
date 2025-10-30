#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

use fltk::{
    app, enums,
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window,
};
use fltk_webview_sys as wv;
use std::{os::raw, sync::Arc};
pub use wv::SizeHint;
pub use wv::Webview;

// Linux path is unified under X11; Wayland embedding is not supported.

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
                    pub fn CoInitializeEx(pvReserved: *mut (), dwCoInit: u32) -> i32;
                    pub fn SendMessageW(
                        hwnd: *mut (),
                        msg: u32,
                        wparam: usize,
                        lparam: isize,
                    ) -> isize;
                }
                const COINIT_APARTMENTTHREADED: u32 = 0x2;
                const WM_SIZE: u32 = 0x0005;
                CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED);
                inner = wv::webview_create(debug as i32, std::ptr::null_mut());
                wv::webview_set_size(inner, win.w(), win.h(), 3);
                let wv_hwnd = wv::webview_get_window(inner);
                // Manually set the webview window as a child and position it
                extern "system" {
                    pub fn SetParent(child: *mut (), parent: *mut ()) -> *mut ();
                    pub fn SetWindowPos(
                        hwnd: *mut (),
                        hwnd_insert_after: *mut (),
                        x: i32,
                        y: i32,
                        cx: i32,
                        cy: i32,
                        flags: u32,
                    ) -> i32;
                    pub fn GetWindowLongW(hwnd: *mut (), index: i32) -> i32;
                    pub fn SetWindowLongW(hwnd: *mut (), index: i32, new_long: i32) -> i32;
                }
                const GWL_STYLE: i32 = -16;
                const WS_CHILD: i32 = 0x40000000;
                const WS_VISIBLE: i32 = 0x10000000;
                const SWP_NOZORDER: u32 = 0x0004;
                const SWP_NOACTIVATE: u32 = 0x0010;
                const SWP_FRAMECHANGED: u32 = 0x0020;
                // Change window style to WS_CHILD to remove decorations
                SetWindowLongW(wv_hwnd as _, GWL_STYLE, WS_CHILD | WS_VISIBLE);
                SetParent(wv_hwnd as _, win.raw_handle() as _);
                SetWindowPos(
                    wv_hwnd as _,
                    std::ptr::null_mut(),
                    0,
                    0,
                    win.w(),
                    win.h(),
                    SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
                );
                win.resize_callback(move |w, _, _, _, _| {
                    wv::webview_set_size(inner, w.w(), w.h(), 3);
                });
                win.resize(win.x(), win.y(), win.w(), win.h());
                win.handle(|w, ev| {
                    if ev == enums::Event::Focus {
                        SetFocus(w.raw_handle() as _);
                        true
                    } else {
                        false
                    }
                });
                let mut topwin =
                    window::Window::from_widget_ptr(win.top_window().unwrap().as_widget_ptr());
                // SetFocus(topwin.raw_handle() as _);
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
                    }
                    _ => false,
                });
            }
            #[cfg(target_os = "macos")]
            {
                pub enum NSWindow {}
                extern "C" {
                    pub fn make_delegate(
                        child: *mut NSWindow,
                        parent: *mut NSWindow,
                        add_menu: i32,
                    );
                    pub fn my_close_win(win: *mut NSWindow);
                }
                let handle = win.raw_handle();
                inner = wv::webview_create(debug as i32, handle as _);
                make_delegate(wv::webview_get_window(inner) as _, handle as _, 1);
                win.resize_callback(move |w, _, _, _, _| {
                    wv::webview_set_size(inner, w.w(), w.h(), 0);
                });
                win.resize(win.x(), win.y(), win.w(), win.h());
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
                    pub fn my_gtk_events_pending() -> i32;
                    pub fn my_get_win(wid: *mut GtkWindow) -> *mut GdkWindow;
                    pub fn my_get_xid(w: *mut GdkWindow) -> u64;
                    pub fn x_init(disp: *mut Display, child: u64, parent: u64);
                    pub fn x_focus(disp: *mut Display, child: u64);
                    pub fn gtk_main_iteration_do(blocking: bool);
                }
                std::env::set_var("GDK_BACKEND", "x11");
                gtk_init(&mut 0, std::ptr::null_mut());
                inner = wv::webview_create(debug as i32, std::ptr::null_mut() as _);
                assert!(!inner.is_null());
                let temp_win = wv::webview_get_window(inner);
                assert!(!temp_win.is_null());
                let temp = my_get_win(temp_win as _);
                assert!(!temp.is_null());
                let xid = my_get_xid(temp as _);
                let flxid = win.raw_handle();

                // Unified X11 path: make child unmanaged and reparent into FLTK window
                x_init(app::display() as _, xid, flxid);
                // Ensure input focus goes to the embedded child when shown
                x_focus(app::display() as _, xid);

                win.resize_callback(move |w, _, _, _, _| {
                    wv::webview_set_size(inner, w.w(), w.h(), 0);
                });
                win.resize(win.x(), win.y(), win.w(), win.h());
                // Set focus to child on mouse press to ensure keystrokes reach WebKit
                let xid_for_focus = xid;
                win.handle(move |_, ev| {
                    if ev == enums::Event::Push {
                        x_focus(app::display() as _, xid_for_focus);
                        true
                    } else {
                        false
                    }
                });

                app::add_timeout3(0.016, |handle| {
                    let mut spins = 0;
                    while my_gtk_events_pending() != 0 && spins < 4 {
                        gtk_main_iteration_do(false);
                        spins += 1;
                    }
                    app::repeat_timeout3(0.016, handle);
                });
            }
        }
        assert!(!inner.is_null());
        #[allow(clippy::arc_with_non_send_sync)]
        let inner = Arc::new(inner);
        Webview::from_raw(inner)
    }
}
