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
    wv.navigate("http://wikipedia.com");

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
                use objc::declare::MethodImplementation;
                use objc::runtime::*;
                use objc::{Encode, EncodeArguments};
                extern "C" {
                    pub fn objc_getMetaClass(name: *const std::os::raw::c_char) -> *mut Class;
                }
                extern fn my_method(_this: &Object, _cmd: Sel) -> bool {
                    true
                }
                fn method_type_encoding(
                    ret: &objc::Encoding,
                    args: &[objc::Encoding],
                ) -> std::ffi::CString {
                    let mut types = format!(
                        "{}{}{}",
                        ret.as_str(),
                        <*mut Object>::encode().as_str(),
                        Sel::encode().as_str()
                    );
                    for enc in args {
                        use std::fmt::Write;
                        write!(&mut types, "{}", enc.as_str()).unwrap();
                    }
                    std::ffi::CString::new(types).unwrap()
                }
                fn add_method<F>(cls: *mut Class, sel: Sel, func: F)
                where
                    F: MethodImplementation<Callee = Object>,
                {
                    let encs = F::Args::encodings();
                    let types = method_type_encoding(&F::Ret::encode(), encs.as_ref());
                    unsafe {
                        class_addMethod(
                            cls,
                            sel,
                            func.imp(),
                            types.as_ptr(),
                        );
                    }
                }
                let handle = win.raw_handle();
                let cls2 = objc_getMetaClass("WKWebView\0".as_ptr() as _);
                add_method(
                    cls2,
                    sel!(did_view_resolution_change),
                    my_method as extern fn(&Object, Sel) -> bool,
                );
                inner = wv::webview_create(debug as i32, handle as *mut raw::c_void);
                let cls = object_getClass(wv::webview_get_window(inner) as _) as *mut Class;
                add_method(
                    cls,
                    sel!(did_view_resolution_change),
                    my_method as extern fn(&Object, Sel) -> bool,
                );
            }
            #[cfg(target_os = "linux")]
            {
                pub enum GdkWindow {}
                pub enum GtkWidget {}
                pub enum GtkWindow {}
                pub enum Display {}
                extern "C" {
                    pub fn gtk_init(argc: *mut raw::c_int, argv: *mut *mut raw::c_char);
                    pub fn my_get_win(wid: *mut GtkWindow) -> *mut GdkWindow;
                    pub fn my_get_xid(w: *mut GdkWindow) -> u64;
                    pub fn XReparentWindow(
                        display: *mut Display,
                        w: u64,
                        parent: u64,
                        x: i32,
                        y: i32,
                    );
                    pub fn XMapWindow(display: *mut Display, w: u64);
                    pub fn gtk_widget_set_size_request(wid: *mut GtkWidget, w: i32, h: i32);
                }
                gtk_init(&mut 0, std::ptr::null_mut());
                inner = wv::webview_create(debug as i32, std::ptr::null_mut() as _);
                assert!(!inner.is_null());
                wv::webview_set_size(inner, win.w(), win.h(), 0);
                let temp_win = wv::webview_get_window(inner);
                let temp = my_get_win(temp_win as _);
                assert!(!temp.is_null());
                let xid = my_get_xid(temp as _);
                gtk_widget_set_size_request(temp_win as _, win.x(), win.h());
                XReparentWindow(app::display() as _, xid, win.raw_handle(), win.x(), win.y());
                XMapWindow(app::display() as _, xid);
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

    /// Set the size of the webview window
    pub fn set_size(&mut self, width: i32, height: i32, hints: SizeHint) {
        unsafe { wv::webview_set_size(*self.inner, width, height, hints as i32) }
    }
}
