#![allow(non_camel_case_types)]

// Uses code from https://github.com/webview/webview_rust/blob/dev/src/webview.rs

mod sys;
pub use sys::*;

use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
    sync::Arc,
};

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
    inner: Arc<webview_t>,
}

unsafe impl Send for Webview {}
unsafe impl Sync for Webview {}

impl Drop for Webview {
    fn drop(&mut self) {
        if Arc::strong_count(&self.inner) == 0 {
            unsafe {
                webview_terminate(*self.inner);
                webview_destroy(*self.inner);
            }
        }
    }
}

impl Webview {
    /// Navigate to a url
    pub fn navigate(&self, url: &str) {
        let url = std::ffi::CString::new(url).unwrap();
        unsafe {
            webview_navigate(*self.inner, url.as_ptr() as _);
        }
    }

    /// Set the html content of the weview window
    pub fn set_html(&self, html: &str) {
        // MS Edge chromium based also requires utf-8
        self.navigate(&(String::from("data:text/html;charset=utf-8,") + html));
    }

    /// Injects JavaScript code at the initialization of the new page
    pub fn init(&self, js: &str) {
        let js = CString::new(js).unwrap();
        unsafe {
            webview_init(*self.inner, js.as_ptr());
        }
    }

    /// Evaluates arbitrary JavaScript code. Evaluation happens asynchronously
    pub fn eval(&self, js: &str) {
        let js = CString::new(js).unwrap();
        unsafe {
            webview_eval(*self.inner, js.as_ptr());
        }
    }

    /// Posts a function to be executed on the main thread
    pub fn dispatch<F>(&mut self, f: F)
    where
        F: FnOnce(Webview) + Send + 'static,
    {
        let closure = Box::into_raw(Box::new(f));
        extern "C" fn callback<F>(webview: webview_t, arg: *mut raw::c_void)
        where
            F: FnOnce(Webview) + Send + 'static,
        {
            let webview = Webview {
                inner: Arc::new(webview),
            };
            let closure: Box<F> = unsafe { Box::from_raw(arg as *mut F) };
            (*closure)(webview);
        }
        unsafe { webview_dispatch(*self.inner, Some(callback::<F>), closure as *mut _) }
    }

    /// Binds a native C callback so that it will appear under the given name as a global JavaScript function
    pub fn bind<F>(&self, name: &str, f: F)
    where
        F: FnMut(&str, &str),
    {
        let name = CString::new(name).unwrap();
        let closure = Box::new(f);
        extern "C" fn callback<F: FnMut(&str, &str)>(
            seq: *const raw::c_char,
            req: *const raw::c_char,
            arg: *mut raw::c_void,
        ) {
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
            webview_bind(
                *self.inner,
                name.as_ptr(),
                Some(callback::<F>),
                Box::into_raw(closure) as *mut _,
            )
        };
    }

    /// Unbinds a native C callback so that it will appear under the given name as a global JavaScript function
    pub fn unbind(&self, name: &str) {
        let name = CString::new(name).unwrap();
        let _move = move || unsafe { webview_unbind(*self.inner, name.as_ptr()) };
        _move();
    }

    /// Allows to return a value from the native binding.
    pub fn return_(&self, seq: &str, status: i32, result: &str) {
        let seq = CString::new(seq).unwrap();
        let result = CString::new(result).unwrap();
        unsafe { webview_return(*self.inner, seq.as_ptr(), status, result.as_ptr()) }
    }

    /// Set the size of the webview window
    pub fn set_size(&self, width: i32, height: i32, hints: SizeHint) {
        unsafe { webview_set_size(*self.inner, width, height, hints as i32) }
    }

    /// Create a Webview from an `Arc<webview_t>`
    pub fn from_raw(inner: Arc<webview_t>) -> Webview {
        Self {
            inner
        }
    }
}