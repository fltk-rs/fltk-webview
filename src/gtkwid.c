#include <gdk/gdk.h>
#include <gdk/gdkx.h>
#include <gtk/gtk.h>
#include <X11/Xlib.h>
#include <X11/extensions/Xfixes.h>

long my_get_xid(GdkWindow *win) { return GDK_WINDOW_XID(win); }

GdkWindow *my_get_win(GtkWindow *win) {
  GdkWindow *w = gtk_widget_get_window(GTK_WIDGET(win));
  return w;
}

// copied from https://stackoverflow.com/a/40902444
void xmagic(Display *display, Window childWindowId, Window parentWindowId) {
  // We need to move the child window before reparenting it to avoid some nasty offsets
  XMoveWindow(display, childWindowId, 0, 0);

  // Do the reparenting
  XReparentWindow(display, childWindowId, parentWindowId, 0, 0);

  // Ask the XServer to take ownership back if we die
  XFixesChangeSaveSet(display, childWindowId, SetModeInsert, SaveSetRoot,
                      SaveSetUnmap);

  // We have to explicitly notify the Java child of its location change.
  XEvent client_event;
  XWindowAttributes childAttributes;
  XWindowAttributes parentAttributes;
  XGetWindowAttributes(display, childWindowId, &childAttributes);
  XGetWindowAttributes(display, parentWindowId, &parentAttributes);
  // WindowDimension windowDecorationSize = // Your decoration if applicable

      client_event.type = ConfigureNotify;
  client_event.xconfigure.send_event = True;
  client_event.xconfigure.display = display;
  client_event.xconfigure.event = childWindowId;
  client_event.xconfigure.window = childWindowId;
  // client_event.xconfigure.x = parentAttributes.x + windowDecorationSize.width;
  // client_event.xconfigure.y = parentAttributes.y + windowDecorationSize.height;
  client_event.xconfigure.width = childAttributes.width;
  client_event.xconfigure.height = childAttributes.height;
  client_event.xconfigure.border_width = 0;
  client_event.xconfigure.above = None;
  client_event.xconfigure.override_redirect = True; 

  XSendEvent(display, childWindowId, False, StructureNotifyMask, &client_event);
}