use fltk::{prelude::*, *};
use webview_official as wv;

#[cfg(target_os = "linux")]
use gdk_x11_sys as gdk;

pub fn from(debug: bool, win: &mut window::Window) -> wv::Webview {
    assert!(win.shown());
    win.end();
    win.set_color(enums::Color::White);
    let w;
    unsafe {
        #[cfg(not(target_os = "linux"))]
        {
            use std::os::raw;
            w = wv::Webview::create(
                debug,
                &mut win.raw_handle() as *mut *mut raw::c_void as *mut raw::c_void,
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
            w = wv::Webview::create(debug, Some(&mut *(gtkwid as *mut wv::Window)));
        }
    }
    w
}
