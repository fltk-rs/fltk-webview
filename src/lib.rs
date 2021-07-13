/*!
# fltk-webview

This provides webview functionality for embedded fltk windows. This currently works on Windows:

## Usage

```rust,no_run
use fltk::{app, enums::Event, prelude::*, window};

fn main() {
    let _app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.show();

    // close the app when the main window is closed
    win.set_callback(|_| {
        if app::event() == Event::Close {
            std::process::exit(0);
        }
    });

    let mut wv = fltk_webview::Webview::create(false, &mut wv_win);
    wv.navigate("https://google.com");

    // the webview handles the main loop
    wv.run();
}
```
*/

// Uses code from https://github.com/webview/webview_rust/blob/dev/src/webview.rs

use fltk::{prelude::*, *};
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
    sync::Arc,
};
use webview_official_sys as wv;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub(crate) trait FlString {
    fn safe_new(s: &str) -> CString;
}

impl FlString for CString {
    fn safe_new(s: &str) -> CString {
        match CString::new(s) {
            Ok(v) => v,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SizeHint {
    None = 0,
    Min = 1,
    Max = 2,
    Fixed = 3,
}

/// Webview wrapper
#[derive(Clone)]
pub struct Webview {
    inner: Arc<wv::webview_t>,
}

unsafe impl Send for Webview {}
unsafe impl Sync for Webview {}

impl Drop for Webview {
    fn drop(&mut self) {
        if Arc::strong_count(&self.inner) == 0 {
            unsafe {
                wv::webview_terminate(*self.inner);
                wv::webview_destroy(*self.inner);
            }
        }
    }
}

impl Webview {
    /// Create a Webview from an embedded fltk window. Requires that the window is already shown
    pub fn create(debug: bool, win: &mut window::Window) -> Webview {
        assert!(win.shown());
        win.end();
        win.set_color(enums::Color::White);
        let inner;
        unsafe {
            #[cfg(target_os = "windows")]
            {
                inner = wv::webview_create(
                    debug as i32,
                    &mut win.raw_handle() as *mut *mut raw::c_void as *mut raw::c_void,
                );
            }
            #[cfg(target_os = "macos")]
            {
                use objc::runtime::*;
                let win_view: *mut Object = msg_send![win.raw_handle() as *mut Object, contentView];
                inner = wv::webview_create(debug as i32, std::ptr::null_mut());
                let inner_win = wv::webview_get_window(inner);
                wv::webview_set_size(inner, win.w(), win.h(), 0);
                let inner_view: *mut Object = msg_send![inner_win as *mut Object, contentView];
                let _: () = msg_send![inner_view, removeFromSuperview];
                let _: () = msg_send![win_view, addSubview:inner_view positioned:1 relativeTo:0];
                let _: () = msg_send![inner_view, acceptsFirstResponder];
                let _: () = msg_send![inner_win as *mut Object, close];
            }
            #[cfg(target_os = "linux")]
            {
                use std::os::raw::*;
                pub enum GdkWindow {}
                pub enum GtkWindow {}
                pub enum Display {}
                extern "C" {
                    pub fn gtk_init(argc: *mut i32, argv: *mut *mut c_char);
                    pub fn my_get_win(wid: *mut GtkWindow) -> *mut GdkWindow;
                    pub fn my_get_xid(w: *mut GdkWindow) -> u64;
                    pub fn x_init(disp: *mut Display, child: u64, parent: u64);
                    pub fn x_reparent(disp: *mut Display, child: u64, parent: u64);
                }
                gtk_init(&mut 0, std::ptr::null_mut());
                inner = wv::webview_create(debug as i32, std::ptr::null_mut() as _);
                assert!(!inner.is_null());
                let temp_win = wv::webview_get_window(inner);
                let temp = my_get_win(temp_win as _);
                assert!(!temp.is_null());
                let xid = my_get_xid(temp as _);
                let flxid = win.raw_handle();
                x_init(app::display() as _, xid, flxid);
                if has_program("gnome-shell") {
                    app::add_idle(move || x_reparent(app::display() as _, xid, flxid));
                }
                win.draw(move |w| {
                    wv::webview_set_size(inner, w.w(), w.h(), 0);
                });
            }
        }
        assert!(!inner.is_null());
        let inner = Arc::new(inner);
        Self { inner }
    }

    /// Navigate to a url
    pub fn navigate(&mut self, url: &str) {
        let url = std::ffi::CString::safe_new(url);
        unsafe {
            wv::webview_navigate(*self.inner, url.as_ptr() as _);
        }
    }

    /// Injects JavaScript code at the initialization of the new page
    pub fn init(&mut self, js: &str) {
        let js = CString::safe_new(js);
        unsafe { wv::webview_init(*self.inner, js.as_ptr()) }
    }

    /// Evaluates arbitrary JavaScript code. Evaluation happens asynchronously
    pub fn eval(&mut self, js: &str) {
        let js = CString::safe_new(js);
        unsafe { wv::webview_eval(*self.inner, js.as_ptr()) }
    }

    /// Posts a function to be executed on the main thread
    pub fn dispatch<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Webview) + Send + 'static,
    {
        let closure = Box::into_raw(Box::new(f));
        extern "C" fn callback<F>(webview: wv::webview_t, arg: *mut raw::c_void)
        where
            F: FnOnce(&mut Webview) + Send + 'static,
        {
            let mut webview = Webview {
                inner: Arc::new(webview),
            };
            let closure: Box<F> = unsafe { Box::from_raw(arg as *mut F) };
            (*closure)(&mut webview);
        }
        unsafe { wv::webview_dispatch(*self.inner, Some(callback::<F>), closure as *mut _) }
    }

    /// Binds a native C callback so that it will appear under the given name as a global JavaScript function
    pub fn bind<F>(&mut self, name: &str, f: F)
    where
        F: FnMut(&str, &str),
    {
        let name = CString::safe_new(name);
        let closure = Box::into_raw(Box::new(f));
        extern "C" fn callback<F>(
            seq: *const raw::c_char,
            req: *const raw::c_char,
            arg: *mut raw::c_void,
        ) where
            F: FnMut(&str, &str),
        {
            let seq = unsafe {
                CStr::from_ptr(seq)
                    .to_str()
                    .expect("No null bytes in parameter seq")
            };
            let req = unsafe {
                CStr::from_ptr(req)
                    .to_str()
                    .expect("No null bytes in parameter req")
            };
            let mut f: Box<F> = unsafe { Box::from_raw(arg as *mut F) };
            (*f)(seq, req);
            mem::forget(f);
        }
        unsafe {
            wv::webview_bind(
                *self.inner,
                name.as_ptr(),
                Some(callback::<F>),
                closure as *mut _,
            )
        }
    }

    /// Allows to return a value from the native binding.
    pub fn r#return(&self, seq: &str, status: i32, result: &str) {
        let seq = CString::safe_new(seq);
        let result = CString::safe_new(result);
        unsafe { wv::webview_return(*self.inner, seq.as_ptr(), status, result.as_ptr()) }
    }

    /// Run the main loop of the webview
    pub fn run(&self) {
        unsafe {
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            {
                extern "C" {
                    pub fn gtk_main_iteration_do(val: bool) -> bool;
                }
                while gtk_main_iteration_do(true) {
                    app::check();
                }
            }
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            wv::webview_run(*self.inner) 
        }
    }

    /// Set the size of the webview window
    pub fn set_size(&mut self, width: i32, height: i32, hints: SizeHint) {
        unsafe { wv::webview_set_size(*self.inner, width, height, hints as i32) }
    }
}

fn has_program(prog: &str) -> bool {
    match std::process::Command::new(prog).arg("--version").output() {
        Ok(out) => !out.stdout.is_empty(),
        _ => false,
    }
}