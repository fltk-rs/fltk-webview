/*!
# fltk-webview

This provides webview functionality for embedded fltk windows. This currently works on Windows:

## Usage

```rust,no_run
use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.show();
    
    let mut wv = fltk_webview::Webview::create(false, &mut wv_win);
    wv.navigate("http://google.com");

    app.run().unwrap();
}
```
*/

// Uses code from https://github.com/webview/webview_rust/blob/dev/src/webview.rs

#![allow(unused_imports)]

use webview_official_sys as wv;
use fltk::{prelude::*, *};
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
    sync::Arc,
};

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "linux")]
use gdk_x11_sys as gdk;

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
                let handle = win.raw_handle();
                inner = wv::webview_create(
                    debug as i32,
                    handle as *mut raw::c_void,
                );
            }
            #[cfg(target_os = "linux")]
            {
                gtk_sys::gtk_init(&mut 0, std::ptr::null_mut());
                let mn = gdk_sys::gdk_display_manager_get();
                // fltk::app::display() doesn't work for some reason
                let display = gdk_sys::gdk_display_manager_open_display(
                    mn,
                    concat!(env!("DISPLAY"), "\0").as_ptr() as _,
                );
                let gdkwin =
                    gdk::gdk_x11_window_foreign_new_for_display(display as _, win.raw_handle());
                let gtkwid = gtk_sys::gtk_window_new(0);
                gtk_sys::gtk_widget_set_window(gtkwid, gdkwin);
                inner = wv::webview_create(debug as i32, gtkwid as _);
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
        unsafe { wv::webview_run(*self.inner) }
    }
}
